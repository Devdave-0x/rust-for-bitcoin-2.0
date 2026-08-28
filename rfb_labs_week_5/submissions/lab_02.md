# Lab 02 — Legacy P2PKH construction

## Commands used

```bash
cargo test --test lab_02
```

## Terminal output

```
running 4 tests
test derives_the_expected_p2pkh_address ... ok
test builds_the_standard_p2pkh_lock ... ok
test commits_to_hash160_of_the_public_key ... ok
test puts_unlocking_data_in_scriptsig ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The locking script for a compressed public key is:
`OP_DUP OP_HASH160 <20-byte-HASH160> OP_EQUALVERIFY OP_CHECKSIG`
which in hex starts with `76a914...88ac`.

## Evidence references

All four tests pass. Implementation in `src/labs/lab02_p2pkh.rs`.

## Explanation

**Key identity vs. spend authorization**

The 20-byte HASH160 in a P2PKH output identifies *which* public key is authorized
to spend the coin. Knowing the public key hash proves identity — it confirms the
address belongs to the holder of a particular key pair.

Spend *authorization* is a different claim: the spender must produce a valid ECDSA
signature over the transaction hash using the private key that corresponds to the
committed public key. The scriptSig places both the signature and the full public key
on the stack:

```
<sig> <pubkey>
```

The script engine then:
1. Duplicates the pubkey (`OP_DUP`)
2. Hashes it (`OP_HASH160`)
3. Checks it matches the committed hash (`OP_EQUALVERIFY`) — this is identity
4. Verifies the signature against the pubkey and the transaction hash (`OP_CHECKSIG`) — this is authorization

Without a valid signature, an attacker who knows the pubkey hash (public information)
cannot spend the output, because they lack the corresponding private key.
