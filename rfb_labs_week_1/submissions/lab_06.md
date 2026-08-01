# Lab 06 — Transaction decoding

## Commands used

```bash
# Decode the unconfirmed transaction with full previous-output values (verbosity 2)
bitcoin-cli getrawtransaction "<txid>" 2
```

## Terminal output

```
$ bitcoin-cli getrawtransaction 11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382 2
{
  "txid": "11379f9f95cf4d54d5dd45a01458e20d6127aa4b31426a4aa335e02302219382",
  "vsize": 141,
  "vin": [
    {
      "txid": "51605454cb5ebffca568d99fa68af35a48e1ea4c88ceadc54347a5f0b18fbbe1",
      "vout": 0
    }
  ],
  "vout": [
    {
      "value": 48.99997180,
      "n": 0,
      "scriptPubKey": {
        "hex": "0014d78b4fb9cb84e6fcdf5f81413972aa24f392b838",
        "address": "bcrt1q6795lwwtsnn0eh6ls9qnju42ynee9wpcs0dz75"
      }
    },
    {
      "value": 1.00000000,
      "n": 1,
      "scriptPubKey": {
        "hex": "001430aa57508f9b5a5f6530c0664a1b002180512473",
        "address": "bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz"
      }
    }
  ]
}

Value conservation:
  Input  (51605454...:0):   50.00000000 BTC  (confirmed via listunspent)
  Output 0 — change:        48.99997180 BTC  → bcrt1q6795l... (miner)
  Output 1 — payment:        1.00000000 BTC  → bcrt1qxz49w... (classmate)
  Fee:                        0.00002820 BTC

Check: 50.00000000 = 48.99997180 + 1.00000000 + 0.00002820  ✓
```

## Evidence references

TODO: Screenshot of the decoded transaction output showing inputs, outputs,
and your value-conservation calculation. Name it evidence/lab06_decode.png.

## Explanation

**Value conservation** is a core Bitcoin rule: the sum of all input values must
equal the sum of all output values plus the miner fee. No bitcoin can be
created or destroyed inside a regular transaction — only coinbase transactions
may introduce new supply.

Formally: `Σ inputs = Σ outputs + fee`

The **fee has no dedicated output** because Bitcoin's scripting model has no
built-in concept of a fee field. Instead, the fee is simply the *unassigned
difference* between what is consumed (inputs) and what is explicitly allocated
(outputs). When a miner assembles a block they sum up all inputs and all outputs
for each transaction; the leftover is the fee they collect. This design is
elegant: there is nothing to forge or manipulate about the fee — it is
mathematically enforced by the difference. If a transaction tried to assign more
to outputs than its inputs provide, no honest node would relay or mine it
because the arithmetic would not balance.

In this transaction the miner wallet consumed one or more UTXOs worth more than
1 BTC. It created exactly two outputs: one paying 1 BTC to the receiver's
address, and one returning the surplus (minus the fee) to a change address
controlled by the sender. The difference between total inputs and total outputs
is the fee that will be awarded to the miner who includes this transaction in
a block.
