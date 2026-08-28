# Lab 07 — BIP39 mnemonics and seeds

## Commands used

```bash
cargo test --test lab_07
```

## Terminal output

```
running 4 tests
test rejects_an_invalid_checksum ... ok
test validates_entropy_and_checksum_structure ... ok
test matches_the_published_bip39_seed_vector ... ok
test passphrase_selects_a_different_wallet ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Public test mnemonic structure (12 words):
- Entropy: 128 bits
- Checksum: 4 bits (ENT / 32)
- Total: 132 bits encoded as 12 × 11-bit indexes into the BIP39 word list

Seed from public mnemonic + "TREZOR":
`c55257c360c07c72029aebc1b53c05ed0362ada38ead3e3e9efa3708e53495531f09a6987599d18264c1e1c92f2cf141630c7a3c4ab7c81b2f001698e7463b04`

## Evidence references

All four tests pass. Implementation in `src/labs/lab07_bip39.rs`. Seed matches
the published BIP39 test vector for this mnemonic + "TREZOR" passphrase.

## Explanation

**Checksum is error detection, not encryption**

The BIP39 checksum is the first `ENT/32` bits of the SHA-256 hash of the entropy.
It is appended to the entropy before encoding, giving one final word that changes
if any other word is altered. This lets software detect typos: an invalid checksum
means at least one word is wrong (or the words are in the wrong order).

However, the checksum provides **no secrecy**. Anyone who sees the mnemonic can
verify or regenerate the checksum, because SHA-256 is a public function. The
mnemonic itself is the secret; the checksum only verifies its integrity.

**A forgotten passphrase cannot be recovered from the mnemonic**

BIP39 derives the seed as:
`PBKDF2(HMAC-SHA512, "mnemonic" + passphrase, 2048 rounds)`

The passphrase is concatenated with the constant prefix "mnemonic" and used as
the PBKDF2 salt. PBKDF2 is a one-way function — given the seed and the mnemonic,
there is no way to reverse-compute the passphrase. The only attack is brute force.

This means that mnemonic backup and passphrase backup are *independent*:
- Losing the mnemonic: wallet is unrecoverable regardless of the passphrase.
- Losing the passphrase: wallet is unrecoverable regardless of the mnemonic.
- A different passphrase (including the empty string) produces a completely
  different 512-bit seed and therefore a completely different set of keys and
  addresses.
