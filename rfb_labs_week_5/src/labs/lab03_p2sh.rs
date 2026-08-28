//! Lab 03 — wrap a 2-of-3 multisig rule in P2SH.

use std::str::FromStr;

use bitcoin::opcodes::all::OP_CHECKMULTISIG;
use bitcoin::script::Builder;
use bitcoin::Network;

use crate::error::LabError;
use crate::model::P2shReport;
use crate::LabResult;

fn hex_to_script(hex: &str) -> LabResult<bitcoin::ScriptBuf> {
    let bytes = hex::decode(hex).map_err(|e| LabError::InvalidScript(e.to_string()))?;
    Ok(bitcoin::Script::from_bytes(&bytes).to_owned())
}

/// Build `2 <pub1> <pub2> <pub3> 3 OP_CHECKMULTISIG`.
pub fn build_2_of_3_redeem_script(public_keys: [&str; 3]) -> LabResult<String> {
    let keys: Vec<bitcoin::PublicKey> = public_keys
        .iter()
        .map(|hex| {
            bitcoin::PublicKey::from_str(hex).map_err(|e| LabError::InvalidKey(e.to_string()))
        })
        .collect::<LabResult<_>>()?;

    let script = Builder::new()
        .push_int(2)
        .push_key(&keys[0])
        .push_key(&keys[1])
        .push_key(&keys[2])
        .push_int(3)
        .push_opcode(OP_CHECKMULTISIG)
        .into_script();

    Ok(script.to_hex_string())
}

/// Derive the P2SH address that commits to a redeemScript.
pub fn derive_p2sh_address(redeem_script_hex: &str, network: Network) -> LabResult<String> {
    let script = hex_to_script(redeem_script_hex)?;
    bitcoin::Address::p2sh(&script, network)
        .map(|a| a.to_string())
        .map_err(|e| LabError::InvalidScript(e.to_string()))
}

/// Return the outer `OP_HASH160 <scriptHash> OP_EQUAL` scriptPubKey.
pub fn build_p2sh_script_pubkey(redeem_script_hex: &str) -> LabResult<String> {
    let script = hex_to_script(redeem_script_hex)?;
    // The scriptPubKey bytes are network-independent; use Bitcoin as a carrier.
    bitcoin::Address::p2sh(&script, Network::Bitcoin)
        .map(|a| a.script_pubkey().to_hex_string())
        .map_err(|e| LabError::InvalidScript(e.to_string()))
}

/// Collect the inner script, outer address, and scriptPubKey in one report.
pub fn inspect_p2sh_multisig(public_keys: [&str; 3], network: Network) -> LabResult<P2shReport> {
    let redeem_script_hex = build_2_of_3_redeem_script(public_keys)?;
    let address = derive_p2sh_address(&redeem_script_hex, network)?;
    let script = hex_to_script(&redeem_script_hex)?;
    let script_pubkey_hex = bitcoin::Address::p2sh(&script, network)
        .map(|a| a.script_pubkey().to_hex_string())
        .map_err(|e| LabError::InvalidScript(e.to_string()))?;

    Ok(P2shReport {
        redeem_script_hex,
        address,
        script_pubkey_hex,
    })
}
