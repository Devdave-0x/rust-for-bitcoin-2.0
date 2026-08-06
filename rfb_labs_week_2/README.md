# Rust for Bitcoin 2.0 — Week 2

Build a simplified Bitcoin transaction model while practising structs, enums,
traits, ownership, borrowing, collections, and `Result`-based error handling.

The crate is intentionally incomplete. Search for `TODO` and implement each part;
do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Parts 3–5 in `transaction.rs` and `error.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits and borrowing functions in Parts 6–7.
5. Build the payment example in `main.rs`.
6. Complete UTXO selection and its tests.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in the
submission.

## Written answers

### 1. What is a Bitcoin transaction input?

A transaction input references an existing unspent output (a UTXO) that the sender
wants to spend. It points to that previous output via an outpoint (txid + vout
index), includes a sequence number for relative time-lock control, and in a real
transaction carries a signature script or witness data proving the spender has the
right to use those funds.

### 2. What is a Bitcoin transaction output?

A transaction output creates a new "coin" by locking a specific satoshi amount
behind a spending condition — usually a public-key hash or script. The output stays
unspent on the UTXO set until a future transaction consumes it as an input.

### 3. What is a UTXO?

UTXO stands for Unspent Transaction Output. It is an output from a past transaction
that has not yet been consumed as an input in any subsequent transaction. The entire
Bitcoin UTXO set represents all spendable coins in existence. When you "have bitcoin"
you actually control a set of UTXOs whose spending conditions you can satisfy.

### 4. What does an outpoint identify?

An outpoint uniquely identifies a specific output within the entire blockchain. It
combines the transaction ID (txid) of the transaction that created the output with
the index (vout) of that output within that transaction's output list. Together they
pinpoint exactly one output out of every output ever created on chain.

### 5. How is a transaction fee calculated?

The fee is implicit: `fee = total_input_value − total_output_value`. There is no
explicit fee field in a Bitcoin transaction. Miners collect the difference as
compensation for including the transaction in a block. If the outputs sum to
exactly the inputs, the fee is zero.

### 6. Why use integers rather than floating-point numbers for bitcoin amounts?

Floating-point numbers cannot represent most decimal fractions exactly; `0.1 + 0.2`
in IEEE 754 arithmetic does not equal `0.3`. For a monetary system, even a one-satoshi
rounding error per operation is unacceptable and could be exploited. Satoshi amounts
are always whole numbers, so a `u64` stores them exactly with no rounding at all.
One bitcoin is exactly 100,000,000 satoshis.

### 7. Why does `total_input_value()` borrow `self`?

It only reads the existing inputs to sum their values — it does not need to modify
or consume the transaction. A shared reference (`&self`) is therefore sufficient.
Borrowing rather than moving means the caller can continue using the transaction
after the call, and many borrows can coexist simultaneously.

### 8. Why does `add_input()` take `&mut self`?

Adding an input mutates the transaction by pushing a new element into `self.inputs`.
Mutation through a reference requires an exclusive (mutable) borrow. `&mut self`
gives that write access while ensuring no other code is simultaneously reading or
writing the same transaction.

### 9. What happens when an input is moved into a transaction?

Calling `add_input(input)` transfers ownership of the `InputKind` value into the
transaction's `inputs` Vec. After the call the original binding (`input`) is no
longer valid; the compiler enforces this and will reject any subsequent use of it.
The transaction now owns the data and is responsible for dropping it.

### 10. Why is `Result` preferable to `panic!` for validation failures?

`panic!` terminates the thread immediately and cannot be recovered from in normal
code. Invalid user input or a malformed transaction are *expected* situations —
they should be handled, not crash the program. `Result` makes the failure explicit
in the type system: callers are forced to acknowledge it and decide what to do
(display an error, retry, log, etc.). This also makes the code testable without
catching panics.

### 11. How do enums help model regular and coinbase inputs?

An enum lets a single `InputKind` type represent two fundamentally different
structures — `Regular` (carrying an `OutPoint`, value, and sequence) and `Coinbase`
(carrying a block height and reward) — without any dynamic dispatch or nullable
fields. The compiler then requires every `match` on `InputKind` to handle both
variants. There is no way to silently ignore one case; every code path that touches
an input must be explicit about which kind it is dealing with.

### 12. How does the `BitcoinValue` trait reduce duplication?

Without the trait, code that needs a value — whether from an output or an input —
would need separate functions for each type. With `BitcoinValue`, a single function
that accepts `impl BitcoinValue` (or `&dyn BitcoinValue`) works uniformly for
`TxOutput`, `InputKind::Regular`, and `InputKind::Coinbase`. The default
`value_in_btc()` method is written once on the trait and is automatically available
to every implementor, so the sat-to-BTC conversion cannot drift between types.

---

## Part 7 — Ownership compiler error

The following snippet was used to trigger the error intentionally:

```rust
let input = InputKind::Regular {
    previous_output: OutPoint { txid: "aa".into(), vout: 0 },
    value: 50_000,
    sequence: u32::MAX,
};
let mut tx = Transaction::new(2, 0);
tx.add_input(input);     // ownership of `input` is moved into the Vec here
println!("{:?}", input); // compile error: use of moved value
```

**Compiler output:**

```text
error[E0382]: borrow of moved value: `input`
  --> src/main.rs:12:22
   |
 5 |     let input = InputKind::Regular {
   |         ----- move occurs because `input` has type `InputKind`, which does not implement the `Copy` trait
...
11 |     tx.add_input(input);
   |                  ----- value moved here
12 |     println!("{:?}", input);
   |                      ^^^^^ value borrowed here after move
```

**Why this happens:** `InputKind` does not implement `Copy` (it contains a `String`
inside `OutPoint`, which allocates on the heap and cannot be trivially duplicated).
When `add_input(input)` is called, Rust moves the value into the function, and the
`Vec` inside the transaction takes ownership. The original binding `input` is now
uninitialised. Any attempt to use it afterwards — even a read — violates Rust's
ownership rules, so the compiler rejects it at compile time, before the program ever
runs.

---

## Design notes

**UTXO selection (Part 9):** The implemented algorithm selects UTXOs in the order
they appear in the input slice until the accumulated total reaches the target. This
is simple and predictable but not optimal for real wallets:

- It may include more UTXOs than necessary (e.g., selecting a 70 k + 50 k pair when
  a single 90 k UTXO would suffice), which increases transaction size and fee.
- A better strategy is **largest-first** selection: sort UTXOs by descending value
  and greedily pick. This minimises the number of inputs used and keeps fees low.
- An even better real-world approach is **Branch and Bound** (used in Bitcoin Core),
  which searches for an exact match that produces no change output, avoiding the
  privacy and fee costs of change entirely.

**Validation ordering:** Checks are ordered from cheapest to most expensive and from
structural (no inputs/outputs) to semantic (value arithmetic). The empty-TXID check
happens after the coinbase mixing check so that a coinbase transaction is never
erroneously failed for having an "empty TXID" (coinbase inputs do not have one).

**`Display` for `Transaction`:** The fee line uses the `fee()` method and explicitly
handles the `Err` case with a readable message rather than unwrapping, so printing
an unbalanced transaction never panics.

---

## Example output

```text
=== Transaction ===
  Version:       2
  Locktime:      0
  Inputs:        2
  Outputs:       2
  Total input:   120000 sats
  Total output:  118000 sats
  Fee:           2000 sats
```
