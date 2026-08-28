# Lab 03 — P2SH 2-of-3 multisig

## Commands used

```bash
cargo test --test lab_03
```

## Terminal output

```
running 4 tests
test builds_a_two_of_three_redeem_script ... ok
test derives_the_committed_p2sh_address ... ok
test builds_the_outer_p2sh_lock ... ok
test reports_both_validation_layers ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

The redeemScript is:
`OP_2 <pub1> <pub2> <pub3> OP_3 OP_CHECKMULTISIG`

The outer scriptPubKey is:
`OP_HASH160 <20-byte-script-hash> OP_EQUAL`  (starts with `a914...87`)

## Evidence references

All four tests pass. Implementation in `src/labs/lab03_p2sh.rs`.

## Explanation

**Why matching the script hash is necessary but not sufficient**

The outer P2SH lock (`OP_HASH160 <hash> OP_EQUAL`) checks that the spender supplies
a redeemScript whose HASH160 matches the committed hash. This proves the spender
knows the *exact* redeemScript that was committed to when the address was created.

However, that alone does not satisfy the spend. After the outer check passes, the
Bitcoin script engine *executes* the supplied redeemScript as if it were a new
script. For 2-of-3 multisig the inner script is:

```
OP_2 <pub1> <pub2> <pub3> OP_3 OP_CHECKMULTISIG
```

The stack must then contain two valid ECDSA signatures from any two of the three
listed public keys. Without those signatures, `OP_CHECKMULTISIG` fails and the
transaction is invalid.

So there are two independent conditions:
1. The redeemScript must hash to the committed value (identity/integrity of the script).
2. The redeemScript conditions must be met — here, two valid signatures (authorization).

Knowing the redeemScript (which is public once any spend is broadcast) gives an
attacker no ability to forge signatures, so both layers are required.
