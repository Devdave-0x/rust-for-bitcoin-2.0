# Lab 08 — BIP32 extended key derivation

## Commands used

```bash
cargo test --test lab_08
```

## Terminal output

```
running 4 tests
test creates_a_test_family_master_xpriv ... ok
test derives_matching_extended_keys ... ok
test xpub_derives_a_normal_public_child ... ok
test distinguishes_hardened_and_normal_paths ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Master xpriv for public test mnemonic (no passphrase, regtest): starts with `tprv`.
Derived xpub at `m/84'/1'/0'`: starts with `tpub`.
`path_contains_hardened_step("m/44'/0'/0'/0/0")` → `true`.
`path_contains_hardened_step("m/0/1/2")` → `false`.

## Evidence references

All four tests pass. Implementation in `src/labs/lab08_bip32.rs`.

## Explanation

**Purpose of the chain code**

The chain code is a 32-byte value stored alongside every extended key (both xpriv
and xpub). During child key derivation it is combined with the parent key material
and the child index via HMAC-SHA512. Without the chain code, knowing a parent
public key reveals nothing about child keys — the chain code is the "secret" that
makes derivation deterministic but unpredictable without the root. This is why an
xpub (public key + chain code + metadata) is more sensitive than a bare public key.

**Watch-only use of xpubs**

An xpub contains the public key and chain code but no private key. Normal (non-hardened)
child public keys can be derived from a parent xpub without any private key material.
A watch-only wallet can hold an xpub, derive all child addresses, and monitor balances
and incoming transactions — without being able to sign or spend. This is the basis of
hardware wallet watch-account setups: the software wallet holds xpubs; the hardware
device holds the xprivs.

**Why hardened children cannot be derived from a parent xpub**

For normal derivation, the HMAC input is `pubkey || index`. For hardened derivation,
the input is `0x00 || privkey || index`. The private key material (`privkey`) is
required. An xpub contains no private key, so hardened derivation from an xpub is
mathematically impossible. This is intentional: hardened children prevent a leaked
child private key from compromising the parent xpriv, because the attacker cannot
reverse-derive the parent key without access to the private component used in the
HMAC.
