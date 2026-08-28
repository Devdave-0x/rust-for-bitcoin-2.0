//! Lab 04 — construct and explain native SegWit P2WPKH.

use std::str::FromStr;

use bitcoin::{CompressedPublicKey, Network};

use crate::error::LabError;
use crate::model::{NativeSegwitSpend, WitnessProgramReport};
use crate::LabResult;

fn parse_compressed(public_key_hex: &str) -> LabResult<CompressedPublicKey> {
    let pubkey = bitcoin::PublicKey::from_str(public_key_hex)
        .map_err(|e| LabError::InvalidKey(e.to_string()))?;
    CompressedPublicKey::try_from(pubkey).map_err(|e| LabError::InvalidKey(e.to_string()))
}

/// Derive a native P2WPKH address from a compressed public key.
pub fn derive_p2wpkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    let compressed = parse_compressed(public_key_hex)?;
    Ok(bitcoin::Address::p2wpkh(&compressed, network).to_string())
}

/// Build the P2WPKH `0 <20-byte-pubkey-hash>` scriptPubKey.
pub fn build_p2wpkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    let compressed = parse_compressed(public_key_hex)?;
    Ok(bitcoin::Address::p2wpkh(&compressed, Network::Bitcoin)
        .script_pubkey()
        .to_hex_string())
}

/// Report the witness version and program committed by P2WPKH.
pub fn witness_program(public_key_hex: &str) -> LabResult<WitnessProgramReport> {
    let compressed = parse_compressed(public_key_hex)?;
    // Extract 20-byte program from the "0014<hash>" scriptPubKey bytes
    let spk = bitcoin::Address::p2wpkh(&compressed, bitcoin::Network::Bitcoin).script_pubkey();
    let program_hex = hex::encode(&spk.as_bytes()[2..]);
    Ok(WitnessProgramReport {
        version: 0,
        program_hex: program_hex.clone(),
        program_length: program_hex.len() / 2,
    })
}

/// Put the signature and public key in witness while leaving ScriptSig empty.
pub fn native_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<NativeSegwitSpend> {
    let pubkey = bitcoin::PublicKey::from_str(public_key_hex)
        .map_err(|e| LabError::InvalidKey(e.to_string()))?;
    Ok(NativeSegwitSpend {
        script_sig_hex: String::new(),
        witness_items: vec![signature_hex.to_owned(), pubkey.to_string()],
    })
}
