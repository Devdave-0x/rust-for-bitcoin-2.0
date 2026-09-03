use anyhow::{anyhow, Context, Result};
use bdk_wallet::{
    bitcoin::{
        bip32::Xpriv,
        Address, Amount, FeeRate, Network,
    },
    rusqlite::Connection,
    KeychainKind, PersistedWallet, SignOptions, Wallet,
};
use bip39::Mnemonic;
use bitcoincore_rpc::{Auth, Client, RpcApi};
use serde::{Deserialize, Serialize};
use std::{fs, str::FromStr};

use crate::config::Config;

const NETWORK: Network = Network::Regtest;

#[derive(Serialize, Deserialize)]
struct DescriptorPair {
    external: String,
    internal: String,
}

// --- helpers ---

fn rpc_client(cfg: &Config) -> Result<Client> {
    Client::new(
        &cfg.rpc_url,
        Auth::UserPass(cfg.rpc_user.clone(), cfg.rpc_pass.clone()),
    )
    .context("Cannot connect to Bitcoin node — check RPC_URL/RPC_USER/RPC_PASS in .env")
}

fn load_descriptors(path: &str) -> Result<DescriptorPair> {
    let raw = fs::read_to_string(path).with_context(|| {
        format!(
            "Descriptor file '{}' not found — run `btc-wallet init` first",
            path
        )
    })?;
    serde_json::from_str(&raw).context("Malformed descriptor file")
}

fn open_wallet(cfg: &Config) -> Result<(PersistedWallet<Connection>, Connection)> {
    let descs = load_descriptors(&cfg.descriptor_file)?;
    let mut conn = Connection::open(&cfg.wallet_db)
        .with_context(|| format!("Cannot open wallet DB at '{}'", cfg.wallet_db))?;

    let wallet = match Wallet::load()
        .descriptor(KeychainKind::External, Some(descs.external.clone()))
        .descriptor(KeychainKind::Internal, Some(descs.internal.clone()))
        .load_wallet(&mut conn)
        .context("Failed to load wallet from DB")?
    {
        Some(w) => w,
        None => Wallet::create(descs.external, descs.internal)
            .network(NETWORK)
            .create_wallet(&mut conn)
            .context("Failed to create wallet in DB")?,
    };

    Ok((wallet, conn))
}

// --- public commands ---

pub fn init(mnemonic_words: Option<String>) -> Result<()> {
    let cfg = Config::from_env()?;

    if fs::metadata(&cfg.descriptor_file).is_ok() {
        println!("Wallet already initialised (descriptor file exists).");
        println!("Delete '{}' to start fresh.", cfg.descriptor_file);
        return Ok(());
    }

    let mnemonic: Mnemonic = match mnemonic_words {
        Some(words) => words.trim().parse().context("Invalid mnemonic")?,
        None => {
            use bip39::rand::thread_rng;
            Mnemonic::generate_in_with(&mut thread_rng(), bip39::Language::English, 12)
                .context("Mnemonic generation failed")?
        }
    };

    println!("Mnemonic (save this offline; do NOT commit it):");
    println!("  {}", mnemonic);
    println!();

    // Derive BIP32 master xprv from seed, then BIP84 wpkh descriptors.
    // coin_type = 1 for regtest (same as testnet per BIP44).
    let seed = mnemonic.to_seed("");
    let xprv = Xpriv::new_master(NETWORK, &seed).context("Cannot derive master xprv")?;
    let external = format!("wpkh({}/84h/1h/0h/0/*)", xprv);
    let internal = format!("wpkh({}/84h/1h/0h/1/*)", xprv);

    let pair = DescriptorPair {
        external: external.clone(),
        internal: internal.clone(),
    };
    fs::write(
        &cfg.descriptor_file,
        serde_json::to_string_pretty(&pair)?,
    )
    .with_context(|| format!("Cannot write descriptor file '{}'", cfg.descriptor_file))?;

    println!("Descriptors saved to '{}'", cfg.descriptor_file);
    println!("  External : {}", external);
    println!("  Internal : {}", internal);
    println!();

    // Initialise the SQLite wallet DB
    let mut conn = Connection::open(&cfg.wallet_db)
        .with_context(|| format!("Cannot open wallet DB at '{}'", cfg.wallet_db))?;
    let _wallet = Wallet::create(external, internal)
        .network(NETWORK)
        .create_wallet(&mut conn)
        .context("Failed to create wallet in DB")?;

    println!("Wallet initialised. DB: '{}'", cfg.wallet_db);
    Ok(())
}

pub fn sync() -> Result<()> {
    use bdk_bitcoind_rpc::Emitter;

    let cfg = Config::from_env()?;
    let rpc = rpc_client(&cfg)?;
    let (mut wallet, mut conn) = open_wallet(&cfg)?;

    println!("Syncing with node at {} …", cfg.rpc_url);

    let cp = wallet.latest_checkpoint();
    let start_height = cp.height();
    let mut emitter = Emitter::new(&rpc, cp, start_height);

    let mut count = 0u32;
    while let Some(em) = emitter.next_block()? {
        wallet.apply_block_connected_to(&em.block, em.block_height(), em.connected_to())?;
        wallet.persist(&mut conn)?;
        count += 1;
    }

    // Include unconfirmed mempool transactions
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let mempool = emitter.mempool()?;
    wallet.apply_unconfirmed_txs(mempool.into_iter().map(|(tx, _)| (tx, now)));
    wallet.persist(&mut conn)?;

    println!("Sync complete — {} new blocks applied.", count);
    Ok(())
}

pub fn balance() -> Result<()> {
    let cfg = Config::from_env()?;
    let (wallet, _) = open_wallet(&cfg)?;
    let bal = wallet.balance();

    println!("Balance (regtest):");
    println!("  Confirmed        : {} sat", bal.confirmed);
    println!("  Trusted pending  : {} sat", bal.trusted_pending);
    println!("  Untrusted pending: {} sat", bal.untrusted_pending);
    println!("  Total            : {} sat", bal.total().to_sat());
    Ok(())
}

pub fn receive() -> Result<()> {
    let cfg = Config::from_env()?;
    let (mut wallet, mut conn) = open_wallet(&cfg)?;
    let info = wallet.reveal_next_address(KeychainKind::External);
    wallet.persist(&mut conn)?;

    println!("Next receiving address (index {}):", info.index);
    println!("  {}", info.address);
    Ok(())
}

pub fn send(to: &str, amount_sat: u64, fee_rate_svb: u64) -> Result<()> {
    let cfg = Config::from_env()?;
    let rpc = rpc_client(&cfg)?;
    let (mut wallet, mut conn) = open_wallet(&cfg)?;

    let recipient = Address::from_str(to)
        .context("Invalid recipient address")?
        .require_network(NETWORK)
        .context("Address network mismatch — use a regtest address")?;

    let amount = Amount::from_sat(amount_sat);
    let fee_rate = FeeRate::from_sat_per_vb(fee_rate_svb)
        .ok_or_else(|| anyhow!("Invalid fee rate: {}", fee_rate_svb))?;

    let mut tx_builder = wallet.build_tx();
    tx_builder
        .add_recipient(recipient.script_pubkey(), amount)
        .fee_rate(fee_rate);
    let mut psbt = tx_builder.finish().context("Failed to build transaction")?;

    wallet
        .sign(&mut psbt, SignOptions::default())
        .context("Failed to sign transaction")?;

    let tx = psbt.extract_tx().context("Failed to extract signed transaction")?;
    let txid = tx.compute_txid();

    rpc.send_raw_transaction(&tx)
        .context("Broadcast failed — is bitcoind running and synced?")?;

    wallet.persist(&mut conn)?;

    println!("Transaction broadcast successfully!");
    println!("  txid     : {}", txid);
    println!("  amount   : {} sat  →  {}", amount_sat, to);
    println!("  fee rate : {} sat/vB", fee_rate_svb);
    Ok(())
}
