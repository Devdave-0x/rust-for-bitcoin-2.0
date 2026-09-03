use anyhow::{bail, Result};
use std::env;

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_pass: String,
    pub wallet_db: String,
    pub descriptor_file: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let rpc_url = env::var("RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:18443".into());
        let rpc_user = env::var("RPC_USER").unwrap_or_else(|_| "user".into());
        let rpc_pass = env::var("RPC_PASS").unwrap_or_else(|_| "pass".into());

        if rpc_user.is_empty() || rpc_pass.is_empty() {
            bail!("RPC_USER and RPC_PASS must be set (in .env or environment)");
        }

        Ok(Self {
            rpc_url,
            rpc_user,
            rpc_pass,
            wallet_db: env::var("WALLET_DB").unwrap_or_else(|_| "wallet.db".into()),
            descriptor_file: env::var("DESCRIPTOR_FILE")
                .unwrap_or_else(|_| "wallet_descriptors.json".into()),
        })
    }
}
