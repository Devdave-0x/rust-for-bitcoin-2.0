# Lab 04 — UTXOs and outpoints

## Commands used

```bash
# List all unspent outputs in the miner wallet
bitcoin-cli -rpcwallet=miner listunspent

# (Optional) Get the wallet's reported balance to reconcile against UTXO sum
bitcoin-cli -rpcwallet=miner getbalances
```

## Terminal output

```
$ bitcoin-cli -rpcwallet=miner listunspent
[
  ...
  {
    "txid": "51605454cb5ebffca568d99fa68af35a48e1ea4c88ceadc54347a5f0b18fbbe1",
    "vout": 0,
    "address": "bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6",
    "label": "mining",
    "scriptPubKey": "00147ab5b7aa015155053bfc3b6cdb641ec3f696e918",
    "amount": 50.00000000,
    "confirmations": 200,
    "spendable": true,
    "solvable": true
  },
  ... (102 UTXOs total, all 50 BTC, all spendable)
]

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 5100.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 3600.00000000
  }
}

Outpoint of selected UTXO:
  txid: 51605454cb5ebffca568d99fa68af35a48e1ea4c88ceadc54347a5f0b18fbbe1
  vout: 0

Sum of spendable UTXOs: 102 × 50.0 BTC = 5100.00000000 BTC
Reconciles with trusted balance: 5100.00000000 BTC ✓
```

## Evidence references

TODO: Screenshot of the listunspent output and the getbalances output
side by side. Name it evidence/lab04_utxos.png.

## Explanation

A **UTXO** (Unspent Transaction Output) is the fundamental unit of value in
Bitcoin. Every bitcoin you own exists as one or more UTXOs — discrete chunks
of value locked to a specific address. Unlike a bank account, Bitcoin has no
running balance entry; your "balance" is simply the sum of all UTXOs your
wallet controls.

An **outpoint** is the unique coordinate that identifies a specific UTXO within
the entire blockchain: `txid:vout`. The `txid` is the hash of the transaction
that created the output, and `vout` is the zero-based index of that output
within the transaction. Together they form a globally unique pointer to a single
coin. When you spend a UTXO, you reference it by its outpoint in the new
transaction's input, and Bitcoin Core marks it as spent so it can never be used
again.

A **wallet balance is not an account entry** because Bitcoin has no concept of
accounts. The node does not store "Alice has X BTC" anywhere. Instead, the
wallet scans the UTXO set for outputs whose locking scripts match addresses it
controls, then sums those amounts. The "balance" is a derived number computed
on demand — if you deleted your wallet file and reimported the keys, the wallet
would scan the chain, find the same UTXOs, and arrive at the identical balance.
This is fundamentally different from a bank that records your balance as a
database entry.
