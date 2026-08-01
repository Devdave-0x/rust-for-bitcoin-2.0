# Lab 05 — Broadcast and mempool

## Commands used

```bash
# Send 1 BTC from miner to the classmate address — do NOT mine after this
bitcoin-cli -rpcwallet=miner sendtoaddress "<classmate-address>" 1

# Confirm the TXID appears in the local mempool
bitcoin-cli getrawmempool

# Check the sender's view of the transaction (0 confirmations expected)
bitcoin-cli -rpcwallet=miner gettransaction "<txid>"

# Check the receiver's pending balance
bitcoin-cli -rpcwallet=receiver getbalances
```

## Terminal output

```
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz 1
11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382

$ bitcoin-cli getrawmempool
[
  "11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382",
  "c50fdd3ddc6f3c3cc48def282efd12f54102432902626d662f06b0ec94648640"
]

$ bitcoin-cli -rpcwallet=miner gettransaction 11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382
{
  "amount": -1.00000000,
  "fee": -0.00002820,
  "confirmations": 0,
  "trusted": true,
  "txid": "11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382",
  "time": 1785576839,
  "timereceived": 1785576839,
  "bip125-replaceable": "yes",
  "details": [
    {
      "address": "bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz",
      "category": "send",
      "amount": -1.00000000,
      "vout": 1,
      "fee": -0.00002820
    }
  ]
}

$ bitcoin-cli -rpcwallet=receiver getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 2.00000000,
    "immature": 0.00000000
  }
}
```

## Evidence references

TODO: Screenshot showing the mempool entry and the receiver's pending balance.
Name it evidence/lab05_mempool.png.

## Explanation

A Bitcoin transaction goes through four distinct states before it is considered
final:

1. **Built and signed** — The wallet selects UTXOs, constructs the transaction
   structure (inputs, outputs, amounts), and signs each input with the
   corresponding private key. At this point the transaction exists only in
   memory inside the wallet software; it has not been shared with anyone.

2. **Broadcast** — The signed transaction is serialised and sent to the Bitcoin
   P2P network (or, in our case, submitted directly to our local node via
   `sendtoaddress`). The node performs basic validity checks (correct signatures,
   no double-spend, sufficient fee) and, if valid, relays it to its peers.
   Broadcast is *not* confirmation — the transaction has no block yet.

3. **Mempool** — Every node that receives and validates the transaction stores
   it in its **mempool** (memory pool), a temporary holding area for
   unconfirmed transactions waiting to be included in a block. The transaction
   can be seen by the network but could still be evicted (if the fee is too low,
   the mempool fills up, or a conflicting transaction is confirmed first).

4. **Confirmed** — A miner includes the transaction in a newly mined block and
   broadcasts that block. Once the block is accepted by the network the
   transaction's outputs become trusted, its inputs are permanently spent, and
   it receives its first confirmation. Each subsequent block adds one more
   confirmation, making reversal progressively harder.

The receiver's wallet shows an `untrusted_pending` balance rather than a
`trusted` balance because the coins have not yet been locked into a block —
the payment could still be reversed by a double-spend until mining confirms it.
