//! Lab 09 — force and audit multi-UTXO coin selection.

use crate::labs::lab04_utxos::list_unspent;
use crate::labs::lab06_decode::{calculate_fee, decode_verbose_transaction, identify_payment_and_change};
use crate::model::{MultiUtxoAudit, OutPoint, Utxo};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Send three separate 0.4 BTC funding transactions and return their TXIDs.
pub fn create_three_funding_transactions<C: RpcClient>(
    client: &C,
    miner_wallet: &str,
    alice_address: &str,
) -> LabResult<Vec<String>> {
    let mut txids = Vec::with_capacity(3);
    for _ in 0..3 {
        let raw = client.call(
            Some(miner_wallet),
            "sendtoaddress",
            &[alice_address.to_owned(), "0.4".to_owned()],
        )?;
        let value = parse_cli_value(&raw)?;
        let txid = value
            .as_str()
            .map(ToOwned::to_owned)
            .ok_or(LabError::Parse("sendtoaddress: expected txid string".to_owned()))?;
        txids.push(txid);
    }
    Ok(txids)
}

/// Return confirmed UTXOs belonging to the supplied address.
pub fn confirmed_utxos_for_address<C: RpcClient>(
    client: &C,
    wallet_name: &str,
    address: &str,
) -> LabResult<Vec<Utxo>> {
    let utxos = list_unspent(client, wallet_name)?;
    Ok(utxos
        .into_iter()
        .filter(|u| u.address.as_deref() == Some(address) && u.confirmations > 0)
        .collect())
}

/// Send 1 BTC from Alice and return the new TXID.
pub fn send_combined_payment<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
) -> LabResult<String> {
    let raw = client.call(
        Some(alice_wallet),
        "sendtoaddress",
        &[receiver_address.to_owned(), "1".to_owned()],
    )?;
    let value = parse_cli_value(&raw)?;
    value
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or(LabError::Parse("sendtoaddress: expected txid string".to_owned()))
}

/// Decode Alice's spend and prove that multiple funding UTXOs were combined.
pub fn audit_multi_utxo_spend<C: RpcClient>(
    client: &C,
    alice_wallet: &str,
    receiver_address: &str,
    funding_utxos: &[Utxo],
) -> LabResult<MultiUtxoAudit> {
    // 1. Send the 1 BTC payment.
    let spend_txid = send_combined_payment(client, alice_wallet, receiver_address)?;

    // 2. Decode the transaction.
    let decoded = decode_verbose_transaction(client, &spend_txid)?;

    // 3. Identify payment and change.
    let payment_and_change = identify_payment_and_change(&decoded, receiver_address)?;

    // 4. Calculate fee and input count.
    let fee = calculate_fee(&decoded)?;
    let spend_input_count = decoded.inputs.len();

    // 5. Record the funding outpoints from the supplied funding UTXOs.
    let funding_outpoints: Vec<OutPoint> = funding_utxos.iter().map(|u| u.outpoint()).collect();

    Ok(MultiUtxoAudit {
        funding_outpoints,
        spend_txid,
        spend_input_count,
        payment_and_change,
        fee,
    })
}
