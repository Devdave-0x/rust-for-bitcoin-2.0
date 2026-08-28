# Lab 05 — Sender compatibility map

## Commands used

```bash
cargo test --test lab_05
```

## Terminal output

```
running 4 tests
test older_p2sh_wallet_accepts_wrapped_but_not_native ... ok
test builds_the_four_format_map ... ok
test selects_the_most_modern_supported_format ... ok
test names_the_required_human_encoding ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Compatibility for a P2SH-era wallet (Base58Check only):

| Output format | Can send? |
|---|---|
| P2PKH | ✓ |
| P2SH-P2WPKH | ✓ |
| P2WPKH (native) | ✗ |
| P2TR | ✗ |

## Evidence references

All four tests pass. Implementation in `src/labs/lab05_compatibility.rs`.

## Explanation

**Why an older wallet accepts `3…` but rejects `bc1q…`**

The key distinction is the *address encoding*, not the output script itself:

- `3…` addresses use **Base58Check**, which is just a number in base 58 with a
  4-byte checksum appended. Any wallet that can construct a Bitcoin transaction
  has understood Base58Check since 2009. The version byte inside signals P2SH
  (`0x05` on mainnet), telling the wallet to build `OP_HASH160 <hash> OP_EQUAL`.
  A wallet from 2014 can do this even if it has no concept of SegWit.

- `bc1q…` addresses use **Bech32** (BIP173), a completely different human-readable
  encoding with a BCH error-detection code and no version byte prefix byte in
  the traditional sense. A wallet that predates BIP173 does not understand how
  to decode Bech32, so it cannot extract the witness program needed to build
  `OP_0 <hash>`. It will typically show an error or refuse the address.

**Sending support vs. spending support**

These are independent. A wallet can *receive* on a native SegWit address (because
it generates the address itself) even if its *sending* code does not support Bech32.
Sending requires decoding an external address string; receiving only requires the
wallet to generate and display its own address. Wrapped SegWit (`3…`) was designed
exactly to bridge this gap: wallets that cannot send Bech32 can still pay wrapped
outputs, letting the recipient enjoy SegWit fee savings.
