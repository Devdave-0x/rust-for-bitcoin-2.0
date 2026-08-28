# Lab 04 — Native P2WPKH

## Commands used

```bash
cargo test --test lab_04
```

## Terminal output

```
running 4 tests
test derives_a_native_regtest_address ... ok
test builds_a_version_zero_witness_lock ... ok
test reports_a_twenty_byte_program ... ok
test leaves_scriptsig_empty_and_uses_witness ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

A regtest P2WPKH address starts with `bcrt1q`. The scriptPubKey is:
`OP_0 <20-byte-pubkey-hash>` — hex prefix `0014`.
ScriptSig is empty; the signature and pubkey go in the witness stack.

## Evidence references

All four tests pass. Implementation in `src/labs/lab04_p2wpkh.rs`.

## Explanation

**How P2WPKH differs from P2PKH and P2SH-wrapped SegWit**

| Property | P2PKH (legacy) | P2SH-P2WPKH (wrapped) | P2WPKH (native) |
|---|---|---|---|
| ScriptSig | `<sig> <pubkey>` | `<0 <hash>>` (push redeemScript) | *empty* |
| Witness | — | `<sig> <pubkey>` | `<sig> <pubkey>` |
| scriptPubKey | `OP_DUP OP_HASH160 <hash> OP_EQUALVERIFY OP_CHECKSIG` | `OP_HASH160 <hash> OP_EQUAL` | `OP_0 <20-byte-hash>` |
| Address prefix (mainnet) | `1` | `3` | `bc1q` |
| Sender requirement | Base58Check | Base58Check | Bech32 |

**P2PKH**: All spend data is in `scriptSig`, which is hashed in full for weight
purposes — each non-witness byte costs 4 weight units.

**P2SH-P2WPKH**: The outer scriptPubKey is P2SH, which older wallets can pay. Inside,
the actual spend data is in the witness. The redeemScript (`OP_0 <hash>`) is in
scriptSig. This allows SegWit semantics while remaining payable by legacy wallets.

**Native P2WPKH**: ScriptSig is entirely empty. The `OP_0 <20-byte-hash>` scriptPubKey
is the most compact form. Witness data is discounted to 1 weight unit per byte
(BIP141), making it cheaper than both alternatives. Senders need Bech32 support.
