//! Lab 02 — construct and explain legacy P2PKH.

use std::str::FromStr;

use bitcoin::Network;

use crate::error::LabError;
use crate::model::P2pkhSpendTemplate;
use crate::LabResult;

fn parse_pubkey(public_key_hex: &str) -> LabResult<bitcoin::PublicKey> {
    bitcoin::PublicKey::from_str(public_key_hex).map_err(|e| LabError::InvalidKey(e.to_string()))
}

/// Derive a P2PKH address from a serialized public key.
pub fn derive_p2pkh_address(public_key_hex: &str, network: Network) -> LabResult<String> {
    let pubkey = parse_pubkey(public_key_hex)?;
    Ok(bitcoin::Address::p2pkh(pubkey, network).to_string())
}

/// Build the P2PKH scriptPubKey for the serialized public key.
pub fn build_p2pkh_script_pubkey(public_key_hex: &str) -> LabResult<String> {
    let pubkey = parse_pubkey(public_key_hex)?;
    Ok(bitcoin::Address::p2pkh(pubkey, Network::Bitcoin)
        .script_pubkey()
        .to_hex_string())
}

/// Return the HASH160 commitment made to the public key.
pub fn committed_pubkey_hash(public_key_hex: &str) -> LabResult<String> {
    let pubkey = parse_pubkey(public_key_hex)?;
    Ok(pubkey.pubkey_hash().to_string())
}

/// Place a signature and public key in the legacy unlocking location.
pub fn p2pkh_spend_template(
    signature_hex: &str,
    public_key_hex: &str,
) -> LabResult<P2pkhSpendTemplate> {
    let pubkey = parse_pubkey(public_key_hex)?;
    Ok(P2pkhSpendTemplate {
        script_sig_items: vec![signature_hex.to_owned(), pubkey.to_string()],
        witness_items: vec![],
    })
}
