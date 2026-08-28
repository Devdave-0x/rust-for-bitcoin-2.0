//! Lab 09 — decode BIP44 paths and derive the selected address.

use std::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{ChildNumber, DerivationPath, Xpriv, Xpub};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::Network;

use crate::error::LabError;
use crate::model::Bip44PathInfo;
use crate::LabResult;

fn ordinal(n: u32) -> &'static str {
    match n {
        0 => "first",
        1 => "second",
        2 => "third",
        3 => "fourth",
        4 => "fifth",
        5 => "sixth",
        6 => "seventh",
        7 => "eighth",
        8 => "ninth",
        9 => "tenth",
        _ => "nth",
    }
}

/// Parse `m / purpose' / coin' / account' / change / index`.
pub fn decode_bip44_path(path: &str) -> LabResult<Bip44PathInfo> {
    let dp = DerivationPath::from_str(path).map_err(|e| LabError::InvalidPath(e.to_string()))?;
    let children: Vec<ChildNumber> = dp.into_iter().copied().collect();

    if children.len() != 5 {
        return Err(LabError::InvalidPath(
            "BIP44 path must have exactly 5 levels".into(),
        ));
    }

    let purpose = match children[0] {
        ChildNumber::Hardened { index } => index,
        _ => return Err(LabError::InvalidPath("purpose must be hardened".into())),
    };
    let coin_type = match children[1] {
        ChildNumber::Hardened { index } => index,
        _ => return Err(LabError::InvalidPath("coin_type must be hardened".into())),
    };
    let account = match children[2] {
        ChildNumber::Hardened { index } => index,
        _ => return Err(LabError::InvalidPath("account must be hardened".into())),
    };
    let change = match children[3] {
        ChildNumber::Normal { index } => index,
        _ => return Err(LabError::InvalidPath("change must be normal".into())),
    };
    let index = match children[4] {
        ChildNumber::Normal { index } => index,
        _ => return Err(LabError::InvalidPath("index must be normal".into())),
    };

    Ok(Bip44PathInfo {
        purpose,
        coin_type,
        account,
        change,
        index,
    })
}

/// Translate a decoded path into a concise English explanation.
pub fn describe_bip44_path(info: &Bip44PathInfo) -> String {
    let chain = if info.change == 0 {
        "receive"
    } else {
        "change"
    };
    format!(
        "BIP{purpose} path: coin_type={coin}, {ord_acct} account, {chain} chain, {ord_idx} address",
        purpose = info.purpose,
        coin = info.coin_type,
        ord_acct = ordinal(info.account),
        chain = chain,
        ord_idx = ordinal(info.index),
    )
}

/// Return the same BIP44 path with only its final address index changed.
pub fn with_address_index(path: &str, new_index: u32) -> LabResult<String> {
    let dp = DerivationPath::from_str(path).map_err(|e| LabError::InvalidPath(e.to_string()))?;
    let mut children: Vec<ChildNumber> = dp.into_iter().copied().collect();
    if children.is_empty() {
        return Err(LabError::InvalidPath("path has no child components".into()));
    }
    *children.last_mut().unwrap() = ChildNumber::Normal { index: new_index };
    let new_dp = DerivationPath::from(children);
    Ok(format!("m/{}", new_dp))
}

/// Derive the P2PKH address selected by a BIP44 path.
pub fn derive_bip44_address(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<String> {
    let m = mnemonic
        .parse::<Mnemonic>()
        .map_err(|e| LabError::InvalidMnemonic(e.to_string()))?;
    let seed = m.to_seed(passphrase);
    let secp = Secp256k1::new();
    let master =
        Xpriv::new_master(network, &seed).map_err(|e| LabError::Derivation(e.to_string()))?;
    let dp = DerivationPath::from_str(path).map_err(|e| LabError::InvalidPath(e.to_string()))?;
    let child = master
        .derive_priv(&secp, &dp)
        .map_err(|e| LabError::Derivation(e.to_string()))?;
    let pubkey = bitcoin::PublicKey::new(child.private_key.public_key(&secp));
    Ok(bitcoin::Address::p2pkh(pubkey, network).to_string())
}
