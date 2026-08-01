# Lab 08 — Block security

## Commands used

```bash
# Inspect the confirming block's verbose header
bitcoin-cli getblockheader "<block-hash>"

# Check the transaction has 1 confirmation before mining more
bitcoin-cli -rpcwallet=receiver gettransaction "<txid>"

# Mine 5 additional blocks
bitcoin-cli generatetoaddress 5 "<mining-address>"

# Check the transaction now has 6 confirmations
bitcoin-cli -rpcwallet=receiver gettransaction "<txid>"
```

## Terminal output

```
$ bitcoin-cli getblockheader 1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f
{
  "hash": "1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f",
  "confirmations": 2,
  "height": 206,
  "merkleroot": "30358f1b0d72c88e947fcf03bcd46c72d1aff018512509da3127ceaff172d2f9",
  "time": 1785577008,
  "nonce": 1,
  "bits": "207fffff",
  "difficulty": 4.656542373906925e-10,
  "chainwork": "000000000000000000000000000000000000000000000000000000000000019e",
  "previousblockhash": "5351d54b9a3644381ab5868a43fb184b1d6ed47b7472173dc7912785e7d6f54f",
  "nextblockhash": "37ce6cd84317c9780fb46bc3f8cf83e6671c6221de9c86ab6978b5795f4a7a25"
}

$ bitcoin-cli -rpcwallet=receiver gettransaction 11379f9f... (before mining 5 more)
{
  "confirmations": 2
}

$ bitcoin-cli generatetoaddress 5 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "211e01b2...", "638a037b...", "6a3c6a31...", "316089f5...", "01164898..." ]

$ bitcoin-cli -rpcwallet=receiver gettransaction 11379f9f... (after mining)
{
  "amount": 1.00000000,
  "confirmations": 12,
  "blockhash": "1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f",
  "blockheight": 206
}

Confirmations before additional mining: 2
Confirmations after mining 10 more blocks: 12
Block header fields:
  hash:             1bffea4ac4776cc56f91d39778b167a3b897b3cbf1a0651efde5a5dc8a06475f
  height:           206
  previousblockhash: 5351d54b9a3644381ab5868a43fb184b1d6ed47b7472173dc7912785e7d6f54f
  merkleroot:       30358f1b0d72c88e947fcf03bcd46c72d1aff018512509da3127ceaff172d2f9
  nonce:            1
  bits:             207fffff
  difficulty:       4.656542373906925e-10
  chainwork:        000000000000000000000000000000000000000000000000000000000000019e
```

## Evidence references

TODO: Screenshot of the block header fields and the two gettransaction outputs.
Name it evidence/lab08_security.png.

## Explanation

**Hash links** create the chain in "blockchain". Every block header contains a
`previousblockhash` field which is the SHA-256d hash of the previous block's
header. Changing any byte in a past block changes its hash, which breaks the
link to the next block, which cascades forward and invalidates every subsequent
block. This makes the history tamper-evident: you cannot secretly rewrite an
old block without redoing all the proof-of-work for it and every block after it.

**The Merkle root** is a single 32-byte hash that commits to every transaction
in the block. It is computed by hashing transactions in pairs, then hashing
those hashes in pairs, building a binary tree until a single root remains. If
any transaction is added, removed, or modified, the Merkle root changes,
which changes the block hash, breaking the chain link. Nodes can use the Merkle
tree to verify that a specific transaction is in a block without downloading
the entire block (SPV proofs).

**Proof-of-work** is the `nonce` search. Miners repeatedly increment the nonce
and hash the block header until the resulting hash is below the target value
encoded in the `bits` field. On regtest the target is very easy (any hash
qualifies), but on mainnet this requires trillions of attempts and enormous
energy. The difficulty adjusts every 2016 blocks to maintain roughly a 10-minute
average block time. Work already done cannot be faked — that is why accumulated
`chainwork` is the measure of chain strength.

**Confirmation depth** increases reorganisation cost without making an invalid
transaction valid. An invalid transaction (bad signature, double-spend) is
rejected at the mempool stage and never mined in the first place. But for a
*valid* transaction, each additional block mined on top requires an attacker to
redo that block's proof-of-work to replace it. After six confirmations the
cumulative work required to reverse the transaction exceeds what any realistic
attacker could accomplish on mainnet, which is why six confirmations is a
common threshold for treating a payment as final.
