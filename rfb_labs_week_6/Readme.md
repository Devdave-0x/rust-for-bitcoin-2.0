# Assignment: Building a Bitcoin Wallet in Rust

## Goal

Build a functioning Bitcoin wallet in Rust (regtest) that demonstrates you can use the libraries covered in class effectively.

## Minimum Requirements

Your wallet must be able to:

1. **Generate or import keys** and derive a wallet from a descriptor.
2. **Generate addresses** from both an external (receiving) and internal (change) keychain.
3. **Track UTXOs and calculate balance** for the wallet.
4. **Persist wallet state locally** (e.g. with SQLite) so the wallet can be closed and reopened without losing track of its own state.
5. **Construct, sign, and broadcast a transaction** on testnet.
6. **Connect to a Bitcoin node** (via `bitcoincore-rpc`) to sync wallet state or broadcast transactions, i.e., your wallet should not be purely offline.

## Stretch Goals

Pick any of these if you want to push further:

- Support multiple descriptor types (e.g. compare `wpkh` vs `tr` Taproot)
- Build a simple CLI so a user can check balance, get a new address, and send funds without editing code
- Handle coin selection explicitly rather than relying on defaults
- Add basic error handling/logging that would make this usable by someone other than you
- Explain (in your README) a scenario where you reached for raw `rust-bitcoin` instead of BDK, and demonstrate it with a small code example

## Acceptance Criteria

1. **A PR TO THE RFB GITHUB** (source code, `Cargo.toml`, etc.) as a zip or a link to a repository.
2. **A README** that includes:
   - How to run your wallet (setup instructions, any node/config required)
   - A short explanation (project or descriptor structure, and why you chose it)
   - Which libraries you used where, and why (e.g. "I used `bitcoincore-rpc` for X, and BDK for Y, because...")
   - Any known limitations or things you'd improve with more time

## Constraints

- Testnet or regtest only.
- Do not hardcode private keys or seed phrases in files you submit, use a `.env`, config file, or generate fresh test keys. (This is also good practice for real-world Bitcoin development.)
- You may use any crates that support your chosen libraries (e.g. `dotenv`, `clap`, `tokio`), but the wallet logic itself should go through `rust-bitcoin`, `bitcoincore-rpc`, and/or BDK.

## Reference Material

- rust-bitcoin docs: https://docs.rs/bitcoin/0.32.102/bitcoin/index.html
- Bitcoin Core RPC reference: https://developer.bitcoin.org/reference/rpc/
- BDK Wallet docs: https://docs.rs/bdk_wallet/latest/bdk_wallet/index.html

---

## Grading Rubric (100 points)

| Category | Criteria |
|---|---|
| **Correctness — Core Functionality** | Wallet generates keys/addresses correctly from a descriptor; external and internal keychains are properly separated |
| **Correctness — UTXO & Balance Tracking** | Wallet accurately tracks UTXOs and reports correct balance after syncing |
| **Correctness — Transactions** | Wallet successfully creates, signs, and broadcasts a transaction on testnet/regtest; txid is verifiable |
| **Persistence** | Wallet state survives a restart (SQLite or equivalent) without needing to re-derive/re-sync from scratch |
| **Node Integration** | Wallet correctly connects to and communicates with a Bitcoin node (RPC or BDK-supported connection) |
| **Architecture & Library Use** | Sensible use of rust-bitcoin / bitcoincore-rpc / BDK together, student can justify *why* each was used where|
| **Code Quality** | Reasonably organized, readable, compiles cleanly, handles at least basic errors (doesn't just panic on bad input) |
| **README & Documentation** | Clear setup instructions, explains design decisions, includes proof of a working transaction |
| **Stretch Goals (bonus)** | Any stretch goal attempted and working |

## Timeline

- **Due:** 5th Septermber 2026

---

## Implementation

### How to Run

#### 1. Prerequisites

- Rust (stable)
- Bitcoin Core in regtest mode

Start Bitcoin Core in regtest:

```bash
bitcoind -regtest -daemonwait \
  -rpcuser=user -rpcpassword=pass \
  -fallbackfee=0.0001
```

#### 2. Configure

Copy `.env.example` to `.env` and fill in your RPC credentials:

```bash
cp .env.example .env
# Edit .env:
# RPC_URL=http://127.0.0.1:18443
# RPC_USER=user
# RPC_PASS=pass
```

#### 3. Build

```bash
cargo build --release
# Binary: ./target/release/btc-wallet
```

#### 4. Initialise Wallet

Generate a fresh 12-word mnemonic and derive BIP84 (native SegWit) descriptors:

```bash
./target/release/btc-wallet init
```

Or import an existing mnemonic:

```bash
./target/release/btc-wallet init --mnemonic "word1 word2 ... word12"
```

This creates:
- `wallet_descriptors.json` — external and internal wpkh descriptors (contains xprv — keep private, add to `.gitignore`)
- `wallet.db` — SQLite database for wallet state

#### 5. Get a Receiving Address

```bash
./target/release/btc-wallet receive
# Next receiving address (index 0):
#   bcrt1q...
```

#### 6. Fund the Wallet (regtest)

Mine some blocks to your wallet address:

```bash
bitcoin-cli -regtest generatetoaddress 101 bcrt1q...
```

#### 7. Sync

Scan the blockchain for transactions that pay to your wallet:

```bash
./target/release/btc-wallet sync
# Sync complete — 101 new blocks applied.
```

#### 8. Check Balance

```bash
./target/release/btc-wallet balance
# Balance (regtest):
#   Confirmed        : 5000000000 sat
#   Trusted pending  : 0 sat
#   Untrusted pending: 0 sat
#   Total            : 5000000000 sat
```

#### 9. Send a Transaction

```bash
./target/release/btc-wallet send bcrt1q<recipient> 100000 --fee-rate 2
# Transaction broadcast successfully!
#   txid     : abc123...
#   amount   : 100000 sat  →  bcrt1q...
#   fee rate : 2 sat/vB
```

---

### Design Decisions

#### Descriptor-based wallet (BIP84 wpkh)

The wallet uses BIP84 native SegWit (`wpkh`) descriptors with derivation path `m/84'/1'/0'/0/*` (external) and `m/84'/1'/0'/1/*` (internal, for change). BIP84 was chosen because:

- Native P2WPKH addresses (`bcrt1q…`) have the lowest transaction weight of the standard address types, minimising fees.
- BDK is descriptor-native — passing a descriptor gives you address generation, UTXO tracking, and PSBT construction without any manual key management.
- The BIP44 coin-type `1` is correct for both regtest and testnet.

#### Library choices

| Library | Used for | Why |
|---|---|---|
| `bdk_wallet 1.x` | Descriptor parsing, address derivation, UTXO selection, PSBT building and signing | BDK handles all wallet-specific logic behind a clean descriptor API. It would be far more error-prone to track UTXOs and build PSBTs by hand using raw `rust-bitcoin`. |
| `bdk_wallet::rusqlite` | SQLite persistence (via the `rusqlite` feature) | BDK 1.x bundles `rusqlite` integration via `bdk_chain`. Enabling the `rusqlite` feature gives `rusqlite::Connection` as a `WalletPersister` with schema management handled automatically. No extra crate needed. |
| `bdk_bitcoind_rpc 0.17` | Block-by-block blockchain sync | Provides the `Emitter` abstraction that pages through blocks from a Bitcoin Core node and delivers them to BDK's `apply_block_connected_to`. This avoids hand-writing block polling and reorg handling. |
| `bitcoincore-rpc 0.19` | Broadcasting signed transactions and node connection | Used directly for `send_raw_transaction`. Also the underlying transport that `bdk_bitcoind_rpc` uses for fetching blocks. |
| `bip39 2.x` | Mnemonic generation and seed derivation | Standard BIP39 implementation; seed bytes fed to `bitcoin::bip32::Xpriv::new_master` to derive the master key. |
| `bitcoin 0.32` | Master xprv derivation, primitive types | Used directly for `Xpriv::new_master` — a case where raw `rust-bitcoin` is the right tool because BDK's higher-level key API abstracts away the step where I want to show derivation explicitly. |

#### When raw rust-bitcoin is the right choice

BDK's descriptor API is excellent for *using* keys, but when you want to *show* the derivation chain (mnemonic → seed → xprv → descriptor), raw `rust-bitcoin` is clearer:

```rust
// Explicit derivation — no BDK abstraction layer
let seed = mnemonic.to_seed("");
let xprv = bitcoin::bip32::Xpriv::new_master(Network::Regtest, &seed)?;
let external_desc = format!("wpkh({}/84h/1h/0h/0/*)", xprv);
```

Using `bdk_wallet::keys::IntoExtendedKey` here would hide the derivation steps, making it harder to understand and audit.

#### No hardcoded keys

Private key material flows only at runtime:

- Mnemonic printed to stdout once on `init`, never stored in a committed file.
- xprv is embedded in `wallet_descriptors.json` which is `.gitignore`d.
- RPC credentials live in `.env` (also `.gitignore`d).

### Architecture

```
src/
  main.rs      — CLI entry point (clap subcommands)
  config.rs    — Load RPC credentials from environment / .env
  wallet.rs    — All wallet operations (init, sync, balance, receive, send)
.env.example   — Template for RPC credentials
.gitignore     — Excludes .env, wallet.db, wallet_descriptors.json
```

### Known Limitations

- **No Taproot support** — adding `tr()` descriptors would be a straightforward extension.
- **Single account** — only `m/84'/1'/0'` (account 0). Multi-account support would require managing multiple descriptor pairs.
- **Sync rescans from tip** — if the wallet DB is lost, syncing from genesis is slow. A checkpoint or birth-height would speed recovery.
- **No fee estimation** — fee rate is a CLI argument; integrating `estimatesmartfee` RPC would be a production improvement.
