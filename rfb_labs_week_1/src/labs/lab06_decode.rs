//! Lab 06 — decode a transaction and prove value conservation.

use crate::model::{DecodedInput, DecodedOutput, DecodedTransaction, OutPoint, PaymentAndChange};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Decode a transaction with enough verbosity to include every spent output's value.
pub fn decode_verbose_transaction<C: RpcClient>(
    client: &C,
    txid: &str,
) -> LabResult<DecodedTransaction> {
    let raw = client.call(None, "getrawtransaction", &[txid.to_owned(), "2".to_owned()])?;
    let value = parse_cli_value(&raw)?;

    let txid_out = value
        .get("txid")
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned)
        .ok_or(LabError::MissingField("txid"))?;

    let vsize = value
        .get("vsize")
        .and_then(|v| v.as_u64())
        .ok_or(LabError::MissingField("vsize"))?;

    let inputs = value
        .get("vin")
        .and_then(|v| v.as_array())
        .ok_or(LabError::MissingField("vin"))?
        .iter()
        .map(|vin| {
            let prev_txid = vin
                .get("txid")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("vin.txid"))?;
            let vout = vin
                .get("vout")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32)
                .ok_or(LabError::MissingField("vin.vout"))?;
            let previous_value = vin
                .get("prevout")
                .and_then(|p| p.get("value"))
                .and_then(|v| v.as_f64())
                .ok_or(LabError::MissingField("vin.prevout.value"))?;
            Ok(DecodedInput {
                previous_output: OutPoint { txid: prev_txid, vout },
                previous_value,
            })
        })
        .collect::<LabResult<Vec<_>>>()?;

    let outputs = value
        .get("vout")
        .and_then(|v| v.as_array())
        .ok_or(LabError::MissingField("vout"))?
        .iter()
        .map(|vout_entry| {
            let vout_index = vout_entry
                .get("n")
                .and_then(|v| v.as_u64())
                .map(|n| n as u32)
                .ok_or(LabError::MissingField("vout.n"))?;
            let value_btc = vout_entry
                .get("value")
                .and_then(|v| v.as_f64())
                .ok_or(LabError::MissingField("vout.value"))?;
            let script_pub_key = vout_entry
                .get("scriptPubKey")
                .ok_or(LabError::MissingField("vout.scriptPubKey"))?;
            let hex = script_pub_key
                .get("hex")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned)
                .ok_or(LabError::MissingField("vout.scriptPubKey.hex"))?;
            let address = script_pub_key
                .get("address")
                .and_then(|v| v.as_str())
                .map(ToOwned::to_owned);
            Ok(DecodedOutput {
                vout: vout_index,
                value: value_btc,
                address,
                script_pub_key_hex: hex,
            })
        })
        .collect::<LabResult<Vec<_>>>()?;

    Ok(DecodedTransaction {
        txid: txid_out,
        inputs,
        outputs,
        vsize,
    })
}

/// Return every previous output consumed by the transaction.
pub fn input_outpoints(transaction: &DecodedTransaction) -> Vec<OutPoint> {
    transaction
        .inputs
        .iter()
        .map(|input| input.previous_output.clone())
        .collect()
}

/// Identify the receiver payment and optional change output.
pub fn identify_payment_and_change(
    transaction: &DecodedTransaction,
    receiver_address: &str,
) -> LabResult<PaymentAndChange> {
    let payment = transaction
        .outputs
        .iter()
        .find(|o| o.address.as_deref() == Some(receiver_address))
        .cloned()
        .ok_or(LabError::Parse(format!(
            "no output found for receiver address {receiver_address}"
        )))?;

    // Treat the first remaining non-OP_RETURN output as change.
    let change = transaction
        .outputs
        .iter()
        .find(|o| {
            o.address.as_deref() != Some(receiver_address)
                && !o.script_pub_key_hex.starts_with("6a")
        })
        .cloned();

    Ok(PaymentAndChange { payment, change })
}

/// Calculate `sum(inputs) - sum(outputs)`.
pub fn calculate_fee(transaction: &DecodedTransaction) -> LabResult<f64> {
    let total_in: f64 = transaction.inputs.iter().map(|i| i.previous_value).sum();
    let total_out: f64 = transaction.outputs.iter().map(|o| o.value).sum();
    let fee = total_in - total_out;
    if fee < 0.0 {
        return Err(LabError::Parse(format!(
            "negative fee: inputs {total_in} < outputs {total_out}"
        )));
    }
    // Round to satoshi precision to eliminate floating-point accumulation error.
    Ok((fee * 1e8).round() / 1e8)
}
