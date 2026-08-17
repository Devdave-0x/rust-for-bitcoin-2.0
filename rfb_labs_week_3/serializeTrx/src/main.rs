use clap::{Arg, ArgAction, Command};
use std::error::Error;

#[derive(Debug)]
struct TxInput {
    prev_txid: Vec<u8>,
    vout: u32,
    script_sig: Vec<u8>,
    sequence: u32,
    witness: Vec<Vec<u8>>,
}

#[derive(Debug)]
struct TxOutput {
    value: u64,
    script_pubkey: Vec<u8>,
}

#[derive(Debug)]
struct Transaction {
    version: i32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
    segwit: bool,
}

// ── validation ────────────────────────────────────────────────────────────────

fn parse_hex(s: &str, field: &str) -> Result<Vec<u8>, String> {
    hex::decode(s).map_err(|e| format!("invalid hex for {field}: {e}"))
}

fn parse_hex_32(s: &str, field: &str) -> Result<Vec<u8>, String> {
    let bytes = parse_hex(s, field)?;
    if bytes.len() != 32 {
        return Err(format!(
            "{field} must be 32 bytes (64 hex chars), got {}",
            bytes.len()
        ));
    }
    Ok(bytes)
}

// ── serialization (unchanged logic) ──────────────────────────────────────────

fn encode_varint(value: usize) -> Vec<u8> {
    match value {
        0..=0xfc => vec![value as u8],
        0xfd..=0xffff => {
            let mut r = vec![0xfd];
            r.extend_from_slice(&(value as u16).to_le_bytes());
            r
        }
        0x10000..=0xffff_ffff => {
            let mut r = vec![0xfe];
            r.extend_from_slice(&(value as u32).to_le_bytes());
            r
        }
        _ => {
            let mut r = vec![0xff];
            r.extend_from_slice(&(value as u64).to_le_bytes());
            r
        }
    }
}

fn serialize_transaction(trx: &Transaction) -> Vec<u8> {
    let mut result = Vec::new();

    result.extend_from_slice(&trx.version.to_le_bytes());

    if trx.segwit {
        result.push(0x00);
        result.push(0x01);
    }

    result.extend_from_slice(&encode_varint(trx.inputs.len()));
    for input in &trx.inputs {
        result.extend_from_slice(&input.prev_txid);
        result.extend_from_slice(&input.vout.to_le_bytes());
        result.extend_from_slice(&encode_varint(input.script_sig.len()));
        result.extend_from_slice(&input.script_sig);
        result.extend_from_slice(&input.sequence.to_le_bytes());
    }

    result.extend_from_slice(&encode_varint(trx.outputs.len()));
    for output in &trx.outputs {
        result.extend_from_slice(&output.value.to_le_bytes());
        result.extend_from_slice(&encode_varint(output.script_pubkey.len()));
        result.extend_from_slice(&output.script_pubkey);
    }

    if trx.segwit {
        for input in &trx.inputs {
            result.extend_from_slice(&encode_varint(input.witness.len()));
            for item in &input.witness {
                result.extend_from_slice(&encode_varint(item.len()));
                result.extend_from_slice(item);
            }
        }
    }

    result.extend_from_slice(&trx.locktime.to_le_bytes());
    result
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── CLI parsing ───────────────────────────────────────────────────────────────

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("serialize-trx")
        .about("Serialize a Bitcoin transaction from command-line arguments")
        .arg(
            Arg::new("version")
                .long("version")
                .short('v')
                .default_value("2")
                .help("Transaction version (integer)"),
        )
        .arg(
            Arg::new("segwit")
                .long("segwit")
                .action(ArgAction::SetTrue)
                .help("Mark transaction as SegWit"),
        )
        .arg(
            Arg::new("locktime")
                .long("locktime")
                .default_value("0")
                .help("Locktime (integer)"),
        )
        // inputs: --input <txid>:<vout>:<sequence>[:<scriptsig_hex>]
        .arg(
            Arg::new("input")
                .long("input")
                .short('i')
                .action(ArgAction::Append)
                .help("Input: <txid_hex>:<vout>:<sequence>[:<scriptsig_hex>]  (repeatable)"),
        )
        // outputs: --output <value_sats>:<scriptpubkey_hex>
        .arg(
            Arg::new("output")
                .long("output")
                .short('o')
                .action(ArgAction::Append)
                .help("Output: <value_sats>:<scriptpubkey_hex>  (repeatable)"),
        )
        // witness per input: --witness <item_hex>,<item_hex>,...
        // one --witness per input, in the same order as --input
        .arg(
            Arg::new("witness")
                .long("witness")
                .short('w')
                .action(ArgAction::Append)
                .help("Witness items for one input: <hex>,<hex>,...  (one per --input, in order)"),
        )
        .get_matches();

    let version: i32 = matches
        .get_one::<String>("version")
        .unwrap()
        .parse()
        .map_err(|_| "version must be an integer")?;

    let segwit = matches.get_flag("segwit");

    let locktime: u32 = matches
        .get_one::<String>("locktime")
        .unwrap()
        .parse()
        .map_err(|_| "locktime must be an unsigned integer")?;

    // parse inputs
    let raw_inputs: Vec<&String> = matches
        .get_many::<String>("input")
        .unwrap_or_default()
        .collect();

    if raw_inputs.is_empty() {
        return Err("at least one --input is required".into());
    }

    let mut inputs = Vec::new();
    for raw in &raw_inputs {
        let parts: Vec<&str> = raw.splitn(4, ':').collect();
        if parts.len() < 3 {
            return Err(format!(
                "invalid input format '{}': expected <txid>:<vout>:<sequence>[:<scriptsig>]",
                raw
            )
            .into());
        }
        let prev_txid = parse_hex_32(parts[0], "txid")?;
        let vout: u32 = parts[1]
            .parse()
            .map_err(|_| format!("vout '{}' must be an integer", parts[1]))?;
        let sequence: u32 = if parts[2].starts_with("0x") || parts[2].starts_with("0X") {
            u32::from_str_radix(&parts[2][2..], 16)
                .map_err(|_| format!("invalid sequence '{}'", parts[2]))?
        } else {
            parts[2]
                .parse()
                .map_err(|_| format!("sequence '{}' must be an integer or 0x hex", parts[2]))?
        };
        let script_sig = if parts.len() == 4 && !parts[3].is_empty() {
            parse_hex(parts[3], "scriptsig")?
        } else {
            vec![]
        };
        inputs.push(TxInput {
            prev_txid,
            vout,
            script_sig,
            sequence,
            witness: vec![],
        });
    }

    // parse outputs
    let raw_outputs: Vec<&String> = matches
        .get_many::<String>("output")
        .unwrap_or_default()
        .collect();

    if raw_outputs.is_empty() {
        return Err("at least one --output is required".into());
    }

    let mut outputs = Vec::new();
    for raw in &raw_outputs {
        let parts: Vec<&str> = raw.splitn(2, ':').collect();
        if parts.len() != 2 {
            return Err(format!(
                "invalid output format '{}': expected <value_sats>:<scriptpubkey_hex>",
                raw
            )
            .into());
        }
        let value: u64 = parts[0]
            .parse()
            .map_err(|_| format!("output value '{}' must be an integer", parts[0]))?;
        let script_pubkey = parse_hex(parts[1], "scriptpubkey")?;
        outputs.push(TxOutput {
            value,
            script_pubkey,
        });
    }

    // parse witness (one entry per input, in order)
    let raw_witnesses: Vec<&String> = matches
        .get_many::<String>("witness")
        .unwrap_or_default()
        .collect();

    if !raw_witnesses.is_empty() && raw_witnesses.len() != inputs.len() {
        return Err(format!(
            "number of --witness entries ({}) must match number of --input entries ({})",
            raw_witnesses.len(),
            inputs.len()
        )
        .into());
    }

    for (i, raw) in raw_witnesses.iter().enumerate() {
        let items: Vec<Vec<u8>> = raw
            .split(',')
            .filter(|s| !s.is_empty())
            .map(|s| parse_hex(s, "witness item"))
            .collect::<Result<_, _>>()?;
        inputs[i].witness = items;
    }

    let trx = Transaction {
        version,
        inputs,
        outputs,
        locktime,
        segwit,
    };

    let serialized = serialize_transaction(&trx);

    println!("Serialized transaction (hex):");
    println!("{}", bytes_to_hex(&serialized));
    println!("\nTransaction size: {} bytes", serialized.len());

    Ok(())
}
