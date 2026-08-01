# Lab 03 — Coinbase maturity

## Commands used

```bash
# Mine one block to the miner address
bitcoin-cli generatetoaddress 1 "<mining-address>"

# Check block height after one block
bitcoin-cli getblockcount

# Inspect balances — reward should be immature
bitcoin-cli -rpcwallet=miner getbalances

# Attempt to spend 1 BTC before maturity — this should fail
bitcoin-cli -rpcwallet=miner sendtoaddress "<classmate-address>" 1

# Mine 100 more blocks to satisfy the maturity rule
bitcoin-cli generatetoaddress 100 "<mining-address>"

# Check block height again (should now be 101)
bitcoin-cli getblockcount

# Inspect balances — first coinbase is now spendable
bitcoin-cli -rpcwallet=miner getbalances
```

## Terminal output

```
$ bitcoin-cli generatetoaddress 1 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "1afcd3365d2d6615480b1ea5158bf5e91387ab4697b0412b91a8c59ff246c924" ]

$ bitcoin-cli getblockcount
5

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 0.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 100.00000000
  }
}

$ bitcoin-cli -rpcwallet=miner sendtoaddress bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz 1
error code: -6
error message:
Insufficient funds

$ bitcoin-cli generatetoaddress 100 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "292123746790edb071e23f8c8271504a3de3086dc8fff7b77fb0e7928c1fde5e", ... (100 hashes) ]

$ bitcoin-cli getblockcount
205

$ bitcoin-cli -rpcwallet=miner getbalances
{
  "mine": {
    "trusted": 5100.00000000,
    "untrusted_pending": 0.00000000,
    "immature": 3600.00000000
  }
}
```

## Evidence references

TODO: Screenshots showing the immature balance at height 1, the error
message, and the spendable balance at height 101. Name them
evidence/lab03_immature.png and evidence/lab03_mature.png.

## Explanation

When a miner successfully mines a block they receive a **coinbase reward** —
newly created bitcoin added to the first (coinbase) transaction in that block.
However, the Bitcoin protocol enforces a rule called **coinbase maturity**:
a coinbase output cannot be spent until it has received at least 100
confirmations. This constant is defined as `COINBASE_MATURITY = 100` in the
Bitcoin Core source code.

The reason for this rule is chain safety. If a block is later orphaned in a
reorganisation, the coinbase reward in that block disappears. If miners could
spend coinbase outputs immediately, a reorganisation could invalidate
transactions that spent coins from a now-orphaned block, causing a cascade of
broken transactions throughout the mempool and chain. The 100-block delay gives
the chain enough time to settle so that only deeply-confirmed coinbase rewards
enter circulation.

Labs conventionally mine **101 blocks** on a fresh chain because at height 1
there is one coinbase output (from block 1) that has exactly 1 confirmation.
After 100 *additional* blocks are mined it has 101 confirmations, which is
greater than 100, so it becomes spendable. At height 101 the coinbase from
block 1 is the only mature output; all subsequent coinbase outputs (blocks 2
through 101) are still immature, which is why the immature balance remains
large while trusted jumps to exactly one block reward (50 BTC on regtest).
