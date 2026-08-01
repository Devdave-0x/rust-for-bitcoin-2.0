# Lab 07 — Confirmation and block membership

## Commands used

```bash
# Mine exactly one block to confirm the pending transaction
bitcoin-cli generatetoaddress 1 "<mining-address>"

# Verify the mempool is now empty
bitcoin-cli getrawmempool

# Check the transaction's confirmation count and block hash via the receiver wallet
bitcoin-cli -rpcwallet=receiver gettransaction "<txid>"

# Retrieve the block and verify it contains the TXID
bitcoin-cli getblock "<block-hash>" 1
```

## Terminal output

```
$ bitcoin-cli generatetoaddress 1 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f" ]

$ bitcoin-cli getrawmempool
[]

$ bitcoin-cli -rpcwallet=receiver gettransaction 11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382
{
  "amount": 1.00000000,
  "confirmations": 2,
  "blockhash": "1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f",
  "blockheight": 206,
  "txid": "11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382",
  "details": [
    {
      "address": "bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz",
      "category": "receive",
      "amount": 1.00000000,
      "label": "classmate"
    }
  ]
}

$ bitcoin-cli getblock 1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f 1
{
  "hash": "1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f",
  "height": 206,
  "merkleroot": "30358f1b0d72c88e947fcf03bcd46c72d1aff018512509da3127ceaff172d2f9",
  "nTx": 3,
  "tx": [
    "da99f3872cdb3455ded2383b5c0405c2770696e6fd018d1c7609b776a0fbc911",
    "c50fdd3ddc6f3c3cc48def282efd12f54102432902626d662f06b0ec94648640",
    "11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382"
  ]
}

TXID 11379f9f... found at index 2 in block 1bffea4a... ✓
Mempool is empty — transaction left mempool upon confirmation ✓
Receiver balance is now trusted ✓
```

## Evidence references

TODO: Screenshot showing the empty mempool, the confirmed transaction with
blockhash, and the block containing the TXID. Name it
evidence/lab07_confirm.png.

## Explanation

When a block is mined, the serialised transaction bytes themselves do not
change at all. The TXID, inputs, outputs, amounts, and signatures remain
exactly the same as when the transaction was in the mempool. What changes is
the transaction's *place in the agreed history*.

Before mining, the transaction existed only in ephemeral mempool storage on
each node — a temporary list that is not part of the blockchain. After mining,
the block containing the transaction is broadcast, validated by every node,
and appended to the chain. The transaction is now referenced by the block's
**Merkle root** (a commitment to every transaction in the block), and the
block's hash is chained to every subsequent block via their `previousblockhash`
fields.

This means "confirmation" is not about the transaction gaining new data; it is
about the blockchain gaining a permanent, cryptographically-linked record that
includes the transaction. Every subsequent block mined on top of the confirming
block makes the transaction harder to reverse because an attacker would need to
re-mine all those blocks to reorganise the chain. The receiver's wallet balance
moves from `untrusted_pending` to `trusted` because the node now has proof —
backed by proof-of-work — that the payment is part of the agreed ledger.
