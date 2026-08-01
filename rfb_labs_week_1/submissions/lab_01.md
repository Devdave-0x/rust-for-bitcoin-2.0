# Lab 01 — Regtest network inspection

## Commands used

```bash
# Inspect chain info (returns chain, blocks, bestblockhash, etc.)
bitcoin-cli getblockchaininfo

# Get current block height
bitcoin-cli getblockcount

# Get best-block hash
bitcoin-cli getbestblockhash
```

## Terminal output

```
$ bitcoin-cli getblockchaininfo
{
  "chain": "regtest",
  "blocks": 3,
  "headers": 3,
  "bestblockhash": "32fd3c3d24ff03d333aea8c639c1e716e6389ea25a369c471bf1f22a5c7858a0",
  "bits": "207fffff",
  "target": "7fffff0000000000000000000000000000000000000000000000000000000000",
  "difficulty": 4.656542373906925e-10,
  "time": 1785576219,
  "mediantime": 1785576216,
  "verificationprogress": 1,
  "initialblockdownload": false,
  "chainwork": "0000000000000000000000000000000000000000000000000000000000000008",
  "size_on_disk": 1184,
  "pruned": false,
  "warnings": []
}

$ bitcoin-cli getblockcount
3

$ bitcoin-cli getbestblockhash
32fd3c3d24ff03d333aea8c639c1e716e6389ea25a369c471bf1f22a5c7858a0
```

## Evidence references

TODO: Add a screenshot of the Polar network running and the terminal output
above. Name it evidence/lab01_network.png.

## Explanation

**Polar** is a desktop application that simplifies spinning up local Bitcoin
development environments. It provides a visual interface for creating nodes,
connecting them, and funding wallets without needing to configure Bitcoin Core
manually. Under the hood it orchestrates Docker containers.

**Docker** is the container runtime Polar relies on. Each Bitcoin Core node
runs inside its own isolated Docker container, giving it a consistent,
reproducible environment regardless of the host operating system, and
preventing node processes from interfering with anything else on the machine.

**Bitcoin Core** is the reference implementation of the Bitcoin protocol. It
validates every block and transaction against the consensus rules, stores the
full blockchain, manages wallets, and exposes an RPC interface (`bitcoin-cli`)
that lets external programs query and control the node.

**Regtest** (regression-test mode) is a private, local chain that you control
entirely. Unlike mainnet or testnet, regtest lets you mine blocks instantly on
demand using fake coins with no real value. Every condition — block height,
wallet balances, mempool state — is under your control, making it ideal for
learning and automated testing without any network delays or real funds at risk.
