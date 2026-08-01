# Lab 09 — Multi-UTXO coin selection

## Commands used

```bash
# Create alice wallet
bitcoin-cli createwallet "alice"

# Generate an address for alice
bitcoin-cli -rpcwallet=alice getnewaddress "alice"

# Send three separate 0.4 BTC payments to alice
bitcoin-cli -rpcwallet=miner sendtoaddress "<alice-address>" 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress "<alice-address>" 0.4
bitcoin-cli -rpcwallet=miner sendtoaddress "<alice-address>" 0.4

# Mine a block to confirm all three funding transactions
bitcoin-cli generatetoaddress 1 "<mining-address>"

# Verify alice owns three distinct UTXOs
bitcoin-cli -rpcwallet=alice listunspent

# Generate a receiver address for the 1 BTC spend
bitcoin-cli -rpcwallet=receiver getnewaddress "alice-payment"

# Alice sends 1 BTC — forces coin selection across multiple UTXOs
bitcoin-cli -rpcwallet=alice sendtoaddress "<receiver-address>" 1

# Decode the spend to inspect inputs and outputs
bitcoin-cli getrawtransaction "<spend-txid>" 2
```

## Terminal output

```
$ bitcoin-cli createwallet "alice"
{ "name": "alice", "warning": "" }

$ bitcoin-cli -rpcwallet=alice getnewaddress "alice"
bcrt1qd9jtletmmknu42h5chz33g4fg9xafwy2rul2zw

$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qd9jtletmmknu42h5chz33g4fg9xafwy2rul2zw 0.4
e1d9be16...
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qd9jtletmmknu42h5chz33g4fg9xafwy2rul2zw 0.4
b62af649...
$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qd9jtletmmknu42h5chz33g4fg9xafwy2rul2zw 0.4
35a789b4...

$ bitcoin-cli generatetoaddress 1 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "..." ]

$ bitcoin-cli -rpcwallet=alice listunspent
[
  { "txid": "ffa57ef5fe957c5a4cd7cd5d93f7c8ec8f4a7c48b48de19a6a7caf08589b3906", "vout": 1, "amount": 0.40000000, "confirmations": 2, "spendable": true },
  { "txid": "35a789b4a3d952dd0e563c51f23c4b7f5982af8ff34c3b896e0801f59af91aa7", "vout": 1, "amount": 0.40000000, "confirmations": 2, "spendable": true },
  { "txid": "f436dc168f4047129a1f992ec7a1dcbad63ac9e06585b176038c02d28b2cd660", "vout": 0, "amount": 0.40000000, "confirmations": 2, "spendable": true },
  ... (6 UTXOs total — funding commands ran twice)
]

$ bitcoin-cli -rpcwallet=alice sendtoaddress bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz 1
216cef886aff9d8ce3c4814282e7a83c8d7ee1a39c098f00e7a0fa73b32e1574

$ bitcoin-cli getrawtransaction 216cef886aff9d8ce3c4814282e7a83c8d7ee1a39c098f00e7a0fa73b32e1574 2
{
  "txid": "216cef886aff9d8ce3c4814282e7a83c8d7ee1a39c098f00e7a0fa73b32e1574",
  "vsize": 276,
  "vin": [
    { "txid": "ffa57ef5fe957c5a4cd7cd5d93f7c8ec8f4a7c48b48de19a6a7caf08589b3906", "vout": 1 },
    { "txid": "35a789b4a3d952dd0e563c51f23c4b7f5982af8ff34c3b896e0801f59af91aa7", "vout": 1 },
    { "txid": "f436dc168f4047129a1f992ec7a1dcbad63ac9e06585b176038c02d28b2cd660", "vout": 0 }
  ],
  "vout": [
    { "value": 1.00000000, "n": 0, "scriptPubKey": { "address": "bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz" } },
    { "value": 0.19994480, "n": 1, "scriptPubKey": { "address": "bcrt1qsysq85lu3makhwr0xyakwwkm8g4t9x2swpyy4p" } }
  ]
}

Value conservation:
  Input 0 (ffa57ef5...:1):  0.40000000 BTC
  Input 1 (35a789b4...:1):  0.40000000 BTC
  Input 2 (f436dc16...:0):  0.40000000 BTC
  Total in:                  1.20000000 BTC

  Output 0 — payment:        1.00000000 BTC  → bcrt1qxz49w... (receiver)
  Output 1 — change:         0.19994480 BTC  → bcrt1qsysq8... (alice change)
  Total out:                 1.19994480 BTC

  Fee:                        0.00005520 BTC

Check: 1.20000000 = 1.00000000 + 0.19994480 + 0.00005520  ✓
Input count: 3  (coin selection combined 3 × 0.4 BTC UTXOs to fund 1 BTC payment)
```

## Evidence references

TODO: Screenshot showing alice's three UTXOs and the decoded multi-input
transaction. Name it evidence/lab09_coin_selection.png.

## Explanation

**Input combination** is necessary here because each of alice's UTXOs is
worth only 0.4 BTC, but the payment target is 1 BTC. No single UTXO is large
enough to cover the payment, so Bitcoin Core's coin selection algorithm must
combine multiple UTXOs as inputs. The wallet selected at least three of the
0.4 BTC UTXOs (total 1.2 BTC), paid 1 BTC to the receiver, and returned
the surplus minus the fee as **change** to an address controlled by alice.

**Change** exists because Bitcoin inputs must be consumed completely — you
cannot spend a fraction of a UTXO. If an input is worth more than the payment
amount, the wallet creates an extra output sending the remainder back to itself.
The **fee** is the unassigned difference between total inputs and total outputs
(`1.2 BTC − 1 BTC − change = fee`), awarded to the miner.

**Privacy implication — common-input ownership heuristic**: When a transaction
has multiple inputs, blockchain analysts assume those inputs are controlled by
the same person, because only the owner of all the private keys can sign every
input. This is called the **common-input-ownership heuristic**. By combining
her three UTXOs, alice revealed to any observer that she controls all three
addresses, potentially linking her entire transaction history. A privacy-
conscious user might instead structure payments to avoid coin consolidation, use
CoinJoin to mix inputs from multiple parties, or use a wallet that implements
coin control to avoid combining identifiable UTXOs.
