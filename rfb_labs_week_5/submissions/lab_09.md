# Lab 09 — BIP44 path decoding and address derivation

## Commands used

```bash
cargo test --test lab_09
```

## Terminal output

```
running 4 tests
test decodes_every_bip44_level ... ok
test explains_zero_based_account_and_chain_type ... ok
test changes_only_the_final_index ... ok
test derives_the_selected_bip44_address ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Decoded path `m/44'/0'/2'/1/5`:
- purpose = 44 (BIP44 P2PKH)
- coin_type = 0 (Bitcoin mainnet)
- account = 2 (third account, zero-based)
- change = 1 (change chain)
- index = 5 (sixth address, zero-based)

`with_address_index("m/44'/0'/2'/1/5", 6)` → `"m/44'/0'/2'/1/6"` ✓

## Evidence references

All four tests pass. Implementation in `src/labs/lab09_bip44.rs`.

## Explanation

**Zero-based account and address indexes**

BIP44 uses zero-based indexing throughout. Account 0 is the first account, account 1
the second, and so on. Address index 0 is the first address on a given branch, index 1
the second. This matches standard programming convention but can confuse users accustomed
to "account 1" meaning the first account.

**Hardened apostrophes (purpose, coin_type, account)**

The first three path levels (`purpose'`, `coin_type'`, `account'`) are hardened
(child numbers ≥ 2³¹). Hardened derivation requires the parent private key, so these
levels cannot be derived from an xpub alone. The hardening means that even if a child
private key for one account leaks, an attacker cannot derive the parent xpriv and
therefore cannot reach other accounts. Only the change and address levels are normal
(non-hardened), allowing xpubs to be exported at the account level for watch-only use.

**Receive vs. change branch**

The fourth level is 0 for the receive (external) chain and 1 for the change (internal)
chain. Receive addresses are shared with senders. Change addresses are generated
internally by the wallet when constructing transactions. Separating them allows a
watch-only wallet to distinguish incoming payments from change returned from one's own
transactions, improving privacy and accounting accuracy.
