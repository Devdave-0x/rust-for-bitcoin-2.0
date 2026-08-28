# Lab 06 — Transaction weight and fees

## Commands used

```bash
cargo test --test lab_06
```

## Terminal output

```
running 4 tests
test calculates_bip141_weight ... ok
test rounds_weight_up_to_virtual_bytes ... ok
test calculates_fee_from_feerate ... ok
test reproduces_the_class_fee_comparison ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Class fee comparison at 50 sat/vB:

| Transaction type | Size (vB) | Fee (sats) |
|---|---|---|
| Legacy P2PKH   | 226 | 11,300 |
| Native P2WPKH  | 141 |  7,050 |
| **Savings**    |     |  **4,250** |

## Evidence references

All four tests pass. Implementation in `src/labs/lab06_weight_fees.rs`.

## Explanation

**Why witness discount is not a simple whole-transaction discount**

BIP141 (SegWit) assigns costs at the byte level, not the transaction level:

- Every **non-witness byte** costs **4 weight units**.
- Every **witness byte** costs **1 weight unit**.

The formula is: `weight = stripped_size × 3 + total_size`

where `stripped_size` is the transaction serialized *without* witness data and
`total_size` includes witness data. The virtual size is `ceil(weight / 4)`.

**Why not simply remove witness bytes from the size?**

If witness bytes were free (weight = 0), miners would have no incentive to limit
witness size, creating potential DoS vectors — huge witness data at no cost to the
sender. The 1-weight-unit-per-byte pricing still charges for witness data, just at
one-quarter the rate of non-witness data. This reflects the fact that witness data
is not stored in the UTXO set (spent output data) and is less costly for full nodes
to serve over time.

**Why not one flat discount per transaction?**

A flat discount would reward transactions that move just one byte to witness just as
much as transactions with large witness stacks, creating an unfair fee structure.
Per-byte pricing ensures the discount is proportional to the actual witness savings.
