//! Lab 10 — prove deterministic recovery across BIP44, BIP49, and BIP84.

use std::str::FromStr;

use bip39::Mnemonic;
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{CompressedPublicKey, Network};

use crate::error::LabError;
use crate::model::{AddressFormat, DerivedAddressSet};
use crate::LabResult;

fn derive_child_pubkey(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    network: Network,
) -> LabResult<bitcoin::PublicKey> {
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
    Ok(bitcoin::PublicKey::new(child.private_key.public_key(&secp)))
}

/// Derive one address from an arbitrary full path and selected script family.
pub fn derive_address_for_path(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<String> {
    let pubkey = derive_child_pubkey(mnemonic, passphrase, path, network)?;
    match format {
        AddressFormat::P2pkh => Ok(bitcoin::Address::p2pkh(pubkey, network).to_string()),
        AddressFormat::P2wpkh => {
            let compressed = CompressedPublicKey::try_from(pubkey)
                .map_err(|e| LabError::InvalidKey(e.to_string()))?;
            Ok(bitcoin::Address::p2wpkh(&compressed, network).to_string())
        }
        AddressFormat::P2sh => {
            // BIP49: P2SH wrapping a P2WPKH redeem script
            let compressed = CompressedPublicKey::try_from(pubkey)
                .map_err(|e| LabError::InvalidKey(e.to_string()))?;
            let redeem_script = bitcoin::Address::p2wpkh(&compressed, network).script_pubkey();
            bitcoin::Address::p2sh(&redeem_script, network)
                .map(|a| a.to_string())
                .map_err(|e| LabError::InvalidScript(e.to_string()))
        }
        _ => Err(LabError::InvalidAddress(
            "unsupported address format".into(),
        )),
    }
}

/// Derive index `n` on the BIP44, BIP49, and BIP84 receive branches.
pub fn derive_address_set(
    mnemonic: &str,
    passphrase: &str,
    account: u32,
    index: u32,
    network: Network,
) -> LabResult<DerivedAddressSet> {
    // For regtest/testnet, coin_type = 1; for mainnet, coin_type = 0.
    let coin = match network {
        Network::Bitcoin => 0,
        _ => 1,
    };

    let bip44 = format!("m/44'/{coin}'/{account}'/0/{index}");
    let bip49 = format!("m/49'/{coin}'/{account}'/0/{index}");
    let bip84 = format!("m/84'/{coin}'/{account}'/0/{index}");

    Ok(DerivedAddressSet {
        bip44_p2pkh: derive_address_for_path(
            mnemonic,
            passphrase,
            &bip44,
            AddressFormat::P2pkh,
            network,
        )?,
        bip49_p2sh_p2wpkh: derive_address_for_path(
            mnemonic,
            passphrase,
            &bip49,
            AddressFormat::P2sh,
            network,
        )?,
        bip84_p2wpkh: derive_address_for_path(
            mnemonic,
            passphrase,
            &bip84,
            AddressFormat::P2wpkh,
            network,
        )?,
    })
}

/// Prove that identical mnemonic, passphrase, path, and network reproduce an address.
pub fn recovery_is_repeatable(
    mnemonic: &str,
    passphrase: &str,
    path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, path, format, network)?;
    Ok(first == second)
}

/// Prove that changing only the final index selects a different address.
pub fn changing_index_changes_address(
    mnemonic: &str,
    passphrase: &str,
    first_path: &str,
    second_path: &str,
    format: AddressFormat,
    network: Network,
) -> LabResult<bool> {
    let first = derive_address_for_path(mnemonic, passphrase, first_path, format, network)?;
    let second = derive_address_for_path(mnemonic, passphrase, second_path, format, network)?;
    Ok(first != second)
}
