# Lab 01 — Address and network identification

## Commands used

```bash
cargo test --test lab_01
cargo test --test lab_01 -- --nocapture
```

## Terminal output

```
running 4 tests
test identifies_human_readable_prefixes ... ok
test maps_regtest_prefixes ... ok
test inspects_a_network_checked_address ... ok
test rejects_an_address_for_the_wrong_network ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Prefix map for regtest:
- P2PKH  → `m/n`   (Base58Check, version byte 0x6f)
- P2SH   → `2`     (Base58Check, version byte 0xc4)
- P2WPKH → `bcrt1q` (Bech32, witness version 0)
- P2TR   → `bcrt1p` (Bech32m, witness version 1)

## Evidence references

All four public tests pass in `tests/lab_01.rs`. Implementation is in
`src/labs/lab01_addresses.rs`.

## Explanation

A prefix tells you the *likely* format and network but is not sufficient validation
on its own because:

1. **Checksum**: Base58Check includes a 4-byte checksum; Bech32/Bech32m include a
   BCH error-detection code. An address with the right prefix but a corrupted body
   would pass a prefix check and silently send funds to an unspendable output.
2. **Network enforcement**: The prefix `m` or `n` is shared by testnet3 and regtest.
   Without calling `require_network`, a regtest address would be accepted on testnet
   and vice versa, causing a transaction broadcast on the wrong chain.
3. **Script type**: `bc1q` indicates P2WPKH *or* P2WSH (both use witness version 0);
   only the decoded program length (20 vs 32 bytes) distinguishes them. A prefix
   check alone cannot tell them apart.

Full validation — parsing the address with `rust-bitcoin`, verifying the checksum,
and calling `require_network` — is always required before constructing a transaction.
