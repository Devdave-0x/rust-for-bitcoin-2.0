//! Lab 10 — observe competing branches and most-work convergence.

use crate::model::{ChainTip, ForkSnapshot, ReorgReport};
use crate::rpc::{parse_cli_value, RpcClient};
use crate::{LabError, LabResult};

/// Read height, best-block hash, and accumulated chainwork from one node.
pub fn get_chain_tip<C: RpcClient>(client: &C) -> LabResult<ChainTip> {
    let raw = client.call(None, "getblockchaininfo", &[])?;
    let value = parse_cli_value(&raw)?;
    Ok(ChainTip {
        height: value
            .get("blocks")
            .and_then(|v| v.as_u64())
            .ok_or(LabError::MissingField("blocks"))?,
        best_block_hash: value
            .get("bestblockhash")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned)
            .ok_or(LabError::MissingField("bestblockhash"))?,
        chainwork: value
            .get("chainwork")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned)
            .ok_or(LabError::MissingField("chainwork"))?,
    })
}

/// Disconnect a peer by its address.
pub fn disconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    client.call(None, "disconnectnode", &[peer_address.to_owned()])?;
    Ok(())
}

/// Reconnect a peer for a one-time synchronization attempt.
pub fn reconnect_peer<C: RpcClient>(client: &C, peer_address: &str) -> LabResult<()> {
    client.call(None, "addnode", &[peer_address.to_owned(), "onetry".to_owned()])?;
    Ok(())
}

/// Compare the private competing tips with the final synchronized tips.
pub fn build_reorg_report(
    common_tip_before_split: &str,
    competing_tips: ForkSnapshot,
    final_tips: ForkSnapshot,
) -> ReorgReport {
    let converged = final_tips.node_a.best_block_hash == final_tips.node_b.best_block_hash
        && final_tips.node_a.height == final_tips.node_b.height;
    ReorgReport {
        common_tip_before_split: common_tip_before_split.to_owned(),
        competing_tips,
        final_tips,
        converged,
    }
}
