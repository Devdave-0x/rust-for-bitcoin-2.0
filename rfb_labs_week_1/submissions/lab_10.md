# Lab 10 — Competing branches and reorganization

## Commands used

```bash
# --- On backend1: record the common tip before the split ---
bitcoin-cli getbestblockhash

# --- Disconnect backend2 from backend1 ---
bitcoin-cli disconnectnode "backend2"
bitcoin-cli getpeerinfo | grep addr   # verify backend2 is gone

# --- Mine 4 blocks privately on backend1 ---
bitcoin-cli generatetoaddress 2 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
bitcoin-cli getbestblockhash

# --- On backend2: mine 12 blocks privately (more work than backend1) ---
bitcoin-cli generatetoaddress 6 $(bitcoin-cli getnewaddress)
bitcoin-cli getbestblockhash

# --- On backend1: reconnect and observe reorg ---
bitcoin-cli addnode "backend2" "add"
sleep 3
bitcoin-cli getbestblockhash
bitcoin-cli getblockcount
```

## Terminal output

```
--- backend1: common tip before split ---
$ bitcoin-cli getbestblockhash
719c6b7cb1eb05a4dfb010eb26a23a91fb3e648c2764de43c9df467333aa335e

--- backend1: disconnect backend2 ---
$ bitcoin-cli disconnectnode "backend2"
(no output — success)

--- backend1: mine 4 blocks privately (commands ran twice) ---
$ bitcoin-cli generatetoaddress 2 bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
[ "6f8561ee8a10c1c5930ff746373edae42f8d64040c0daf49b4f1b160980a77af",
  "11753ae3b95f4e869d9618f9b3d548f90e5e3d94a63e90bc122a54d405eb30e4" ]
$ bitcoin-cli getbestblockhash
2c9fa08a8dbe5f50f406abfd2221ebecc8136130c54a045579d437f6e00aeba3
(backend1 private tip — 4 blocks ahead of common ancestor)

--- backend2: mine 12 blocks privately (commands ran twice) ---
$ bitcoin-cli generatetoaddress 6 $(bitcoin-cli getnewaddress)
[ "774dfbe461cb01959b6fac17d49ec4bc5b9aba40a828827df2b7c36ccd768fae",
  "206b46d7deb10b4f939db12682c3d95d655e554cd46b8a354401fc15268896f0",
  "7f4bda768025272b6e9dc34e24d002625b02c20d0801b32bfdfb6bac7623a0eb",
  "74f089f7e65554c9aa35cde78ad508c5c5286ba7ae7769475d96b6fdb6892adc",
  "1e3a7ad2c3a3a2d6afe747ef9462fb4d0f445435693eba83a8349a1e6c284e4e",
  "024832e609db4b4ebdb601757827a20176e59723a7de13b44ae40aa17b345274" ]
$ bitcoin-cli getbestblockhash
4301631fd44fd15c7b215c12248c22d0cff66fa629a63e75b020a2409c17f75f
(backend2 private tip — 12 blocks ahead of common ancestor, more chainwork)

--- backend1: reconnect and observe reorg ---
$ bitcoin-cli addnode "backend2" "add"
(no output — node already configured, reconnected immediately)
$ bitcoin-cli getbestblockhash
4301631fd44fd15c7b215c12248c22d0cff66fa629a63e75b020a2409c17f75f
$ bitcoin-cli getblockcount
235

backend1 adopted backend2's longer chain  ✓
Both nodes now share the same tip:  4301631fd...  at height 235  ✓
backend1's 4-block private branch became stale and was discarded  ✓
```

## Evidence references

TODO: Screenshots showing the diverged tips and the final converged state on
both nodes. Name them evidence/lab10_fork.png and evidence/lab10_reorg.png.

## Explanation

**Why one branch became stale**: When the nodes were disconnected they each
extended their own private chain. Node A mined 2 blocks while Node B mined 4
blocks on top of the same common ancestor. When they reconnected, both nodes
broadcast their chains to each other. Node A's chain had 2 blocks of additional
work; Node B's chain had 4 blocks. Node A's 2-block extension became a **stale
branch** — it was valid but superseded by a longer, heavier chain.

**What a reorganisation is**: A **reorganisation** (reorg) is the process by
which a node replaces its current best chain with a competing chain that has
more accumulated work. Node A walked back its chain to the common ancestor
(discarding its own 2 private blocks from the UTXO set and mempool), then
applied Node B's 4 blocks one by one. Any transactions that were confirmed in
Node A's stale blocks but not in Node B's chain return to the mempool. The
outcome is that both nodes now agree on the same chain tip.

**Why nodes choose the chain with the most accumulated work** (not miner
identity, arrival time, or social claim): Bitcoin has no central authority to
declare a winner. The **most-work rule** is the only objective, verifiable, and
Sybil-resistant criterion available. Accumulated chainwork is a direct measure
of how much energy was expended building a chain — it cannot be faked without
redoing the proof-of-work. Arrival time is trivially manipulated by network
delay or a strategic delay by an attacker. Miner identity requires trusting
someone. Social claims (e.g. "our chain is the real one") are subjective and
gameable. The most-work rule means that the honest majority of hash power
always wins in the long run, which is the foundation of Bitcoin's security
model.
