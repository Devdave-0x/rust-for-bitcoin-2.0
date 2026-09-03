mod config;
mod wallet;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "btc-wallet", version = "1.0", about = "Bitcoin regtest wallet (BDK + bitcoincore-rpc)")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a new wallet (or pass --mnemonic to import)
    Init {
        /// 12-word BIP39 mnemonic to import (omit to generate a fresh one)
        #[arg(long)]
        mnemonic: Option<String>,
    },
    /// Sync wallet with the Bitcoin node
    Sync,
    /// Show confirmed and pending balance
    Balance,
    /// Get the next unused receiving address
    Receive,
    /// Send funds to an address
    Send {
        /// Recipient address (regtest)
        address: String,
        /// Amount to send in satoshis
        amount: u64,
        /// Fee rate in sat/vB
        #[arg(long, default_value = "2")]
        fee_rate: u64,
    },
}

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();

    match cli.command {
        Command::Init { mnemonic } => wallet::init(mnemonic)?,
        Command::Sync               => wallet::sync()?,
        Command::Balance            => wallet::balance()?,
        Command::Receive            => wallet::receive()?,
        Command::Send { address, amount, fee_rate } => wallet::send(&address, amount, fee_rate)?,
    }

    Ok(())
}
