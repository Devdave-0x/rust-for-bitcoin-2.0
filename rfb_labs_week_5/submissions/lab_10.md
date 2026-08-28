# Lab 10 — Deterministic recovery across BIP44, BIP49, and BIP84

## Commands used

```bash
cargo test --test lab_10
```

## Terminal output

```
running 4 tests
test derives_three_regtest_address_families ... ok
test identical_recovery_inputs_repeat ... ok
test changing_only_the_index_changes_the_address ... ok
test format_selection_changes_the_lock_target ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

From the public test mnemonic (no passphrase), account 0, index 0 on regtest:

| Path | Format | Address prefix |
|---|---|---|
| `m/44'/1'/0'/0/0` | P2PKH | `m` or `n` |
| `m/49'/1'/0'/0/0` | P2SH-P2WPKH | `2` |
| `m/84'/1'/0'/0/0` | P2WPKH | `bcrt1q` |

Derivation is repeatable: same mnemonic + passphrase + path → same address every time.
Changing index 0 to index 1 produces a different address.

## Evidence references

All four tests pass. Implementation in `src/labs/lab10_recovery.rs`.

## Explanation

**Why identical recovery inputs reproduce the same address**

The entire derivation chain is deterministic:

1. `Mnemonic → seed`: PBKDF2 with a fixed passphrase is a pure function. The same
   mnemonic and passphrase always yield the same 512-bit seed.
2. `seed → master xpriv`: HMAC-SHA512("Bitcoin seed", seed) always gives the same
   master key and chain code.
3. `master xpriv → child xpriv at path`: each derivation step is a deterministic
   HMAC-SHA512 over the parent key, chain code, and child index. The same path
   always leads to the same child key.
4. `child xpriv → address`: public key derivation and address encoding are
   deterministic for a given script format and network.

There is no randomness in any step. This is the fundamental promise of HD wallets:
a single mnemonic backup is sufficient to restore every key and address forever.

**Why restoring a wallet also depends on path and script conventions**

If you restore with the correct mnemonic and passphrase but use the wrong derivation
path (e.g., BIP44 instead of BIP84) or the wrong script format (P2PKH instead of
P2WPKH), you derive different addresses. Your funds are in the blockchain under the
*original* addresses; the wrong path derives keys that happen to be unrelated to
those addresses. The wallet will show zero balance.

This is why wallet software records and exports the derivation path alongside the
mnemonic, and why users should document which paths (BIP44/49/84) their wallet uses
before moving funds. The mnemonic alone is not a complete wallet backup without the
path and script-family convention.
