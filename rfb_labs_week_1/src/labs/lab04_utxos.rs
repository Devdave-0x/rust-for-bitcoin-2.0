//! Lab 04 — inspect UTXOs and outpoints.

use crate::model::{OutPoint, Utxo};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Return all UTXOs tracked by the selected wallet.
pub fn list_unspent<C: RpcClient>(client: &C, wallet_name: &str) -> LabResult<Vec<Utxo>> {
    let raw = client.call(Some(wallet_name), "listunspent", &[])?;
    let value = parse_cli_value(&raw)?;
    let array = value
        .as_array()
        .ok_or(LabError::Parse("listunspent: expected array".to_owned()))?;

    array
        .iter()
        .map(|entry| {
            Ok(Utxo {
                txid: entry
                    .get("txid")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned)
                    .ok_or(LabError::MissingField("txid"))?,
                vout: entry
                    .get("vout")
                    .and_then(|v| v.as_u64())
                    .map(|n| n as u32)
                    .ok_or(LabError::MissingField("vout"))?,
                address: entry
                    .get("address")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned),
                script_pub_key: entry
                    .get("scriptPubKey")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned)
                    .ok_or(LabError::MissingField("scriptPubKey"))?,
                amount: entry
                    .get("amount")
                    .and_then(|v| v.as_f64())
                    .ok_or(LabError::MissingField("amount"))?,
                confirmations: entry
                    .get("confirmations")
                    .and_then(|v| v.as_u64())
                    .ok_or(LabError::MissingField("confirmations"))?,
                spendable: entry
                    .get("spendable")
                    .and_then(|v| v.as_bool())
                    .ok_or(LabError::MissingField("spendable"))?,
            })
        })
        .collect()
}

/// Select one spendable UTXO, preferring the one with the most confirmations.
pub fn select_spendable_utxo(utxos: &[Utxo]) -> Option<Utxo> {
    utxos
        .iter()
        .filter(|u| u.spendable)
        .max_by_key(|u| u.confirmations)
        .cloned()
}

/// Convert a UTXO into its unique `txid:vout` coordinate.
pub fn outpoint(utxo: &Utxo) -> OutPoint {
    utxo.outpoint()
}

/// Sum only the spendable UTXOs.
pub fn sum_spendable_utxos(utxos: &[Utxo]) -> f64 {
    utxos.iter().filter(|u| u.spendable).map(|u| u.amount).sum()
}
