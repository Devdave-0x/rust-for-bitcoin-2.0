mod transaction;

use sha2::{Digest, Sha256};
use std::io::{self, Read};
use transaction::{Amount, Input, Output, Transaction, Txid};

fn read_u32(bytes: &mut &[u8]) -> Result<u32, io::Error> {
    let mut buf = [0u8; 4];
    bytes.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn read_u64(bytes: &mut &[u8]) -> Result<u64, io::Error> {
    let mut buf = [0u8; 8];
    bytes.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}

fn read_compact_size(bytes: &mut &[u8]) -> Result<u64, io::Error> {
    let mut first = [0u8; 1];
    bytes.read_exact(&mut first)?;
    match first[0] {
        0..=0xfc => Ok(first[0] as u64),
        0xfd => {
            let mut buf = [0u8; 2];
            bytes.read_exact(&mut buf)?;
            Ok(u16::from_le_bytes(buf) as u64)
        }
        0xfe => {
            let mut buf = [0u8; 4];
            bytes.read_exact(&mut buf)?;
            Ok(u32::from_le_bytes(buf) as u64)
        }
        0xff => {
            let mut buf = [0u8; 8];
            bytes.read_exact(&mut buf)?;
            Ok(u64::from_le_bytes(buf))
        }
    }
}

fn read_txid(bytes: &mut &[u8]) -> Result<Txid, io::Error> {
    let mut buf = [0u8; 32];
    bytes.read_exact(&mut buf)?;
    Ok(Txid::from_bytes(buf))
}

fn read_script(bytes: &mut &[u8]) -> Result<String, io::Error> {
    let len = read_compact_size(bytes)? as usize;
    let mut buf = vec![0u8; len];
    bytes.read_exact(&mut buf)?;
    Ok(hex::encode(buf))
}

fn double_sha256(data: &[u8]) -> [u8; 32] {
    let first = Sha256::digest(data);
    let second = Sha256::digest(first);
    second.into()
}

pub fn decode_transaction(hex_str: &str) -> Result<String, Box<dyn std::error::Error>> {
    let raw = hex::decode(hex_str.trim())?;
    let mut cursor: &[u8] = &raw;

    let version = read_u32(&mut cursor)?;

    // Detect SegWit: marker byte 0x00 followed by flag 0x01
    let mut segwit = false;
    let mut marker_buf = [0u8; 1];
    cursor.read_exact(&mut marker_buf)?;
    if marker_buf[0] == 0x00 {
        let mut flag_buf = [0u8; 1];
        cursor.read_exact(&mut flag_buf)?;
        if flag_buf[0] != 0x01 {
            return Err("Unknown SegWit flag".into());
        }
        segwit = true;
    } else {
        // Not SegWit — put the byte back by reconstructing the slice
        cursor = &raw[4..];
    }

    let input_count = read_compact_size(&mut cursor)? as usize;
    let mut inputs = Vec::with_capacity(input_count);
    for _ in 0..input_count {
        let txid = read_txid(&mut cursor)?;
        let vout = read_u32(&mut cursor)?;
        let script_sig = read_script(&mut cursor)?;
        let sequence = read_u32(&mut cursor)?;
        inputs.push(Input {
            txid,
            vout,
            script_sig,
            sequence,
            witness: vec![],
        });
    }

    let output_count = read_compact_size(&mut cursor)? as usize;
    let mut outputs = Vec::with_capacity(output_count);
    for _ in 0..output_count {
        let sats = read_u64(&mut cursor)?;
        let script_pubkey = read_script(&mut cursor)?;
        outputs.push(Output {
            amount: Amount::from_sat(sats),
            script_pubkey,
        });
    }

    // Read witness data (one stack per input)
    if segwit {
        for input in &mut inputs {
            let item_count = read_compact_size(&mut cursor)? as usize;
            for _ in 0..item_count {
                let len = read_compact_size(&mut cursor)? as usize;
                let mut buf = vec![0u8; len];
                cursor.read_exact(&mut buf)?;
                input.witness.push(hex::encode(buf));
            }
        }
    }

    let lock_time = read_u32(&mut cursor)?;

    // Compute txid: double-SHA256 of the *non-SegWit* serialization, bytes reversed
    let txid_bytes = double_sha256(&raw);
    let txid = Txid::from_bytes(txid_bytes);

    let tx = Transaction {
        txid,
        version,
        inputs,
        outputs,
        lock_time,
        segwit,
    };

    Ok(serde_json::to_string_pretty(&tx)?)
}
