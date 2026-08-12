# Rust for Bitcoin 2.0 — Week 2, Session 4

Build a small lending library while practising structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No
Bitcoin and no external crates — just Rust.

The crate is intentionally incomplete. Search for `TODO` and implement each
part; do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Part 2 in `error.rs`, then Part 3 in `library.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits in Part 4 and the two operations in Parts 5–6.
5. Run the ownership experiments and record the errors.
6. Build the demo in `main.rs`.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in
the submission.

## Written answers

### 1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?

An enum makes impossible states unrepresentable. A struct with `is_on_loan: bool`,
`member_id: Option<u32>`, and `day_borrowed: Option<u32>` allows a combination like
`is_on_loan = false` but `member_id = Some(42)` — a logically broken state the
compiler would happily accept. With the enum, `Available` has no fields at all, so
there is simply no place to store a member id when the item is not on loan. Each
variant carries exactly the data that makes sense for that state and nothing more.

### 2. What does `match` force you to do when a fourth `MediaKind` is added later?

Every `match` on `MediaKind` fails to compile until the new variant is handled.
There is no way to silently ignore it. This turns a missing case from a hidden
runtime bug into a compiler error caught instantly — a useful checklist enforced
automatically.

### 3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?

The `Item` struct owns the title. The caller passes a `String` by value, transferring
ownership into the `Item`. After the call the caller can no longer use that
`String` — the `Item` is now solely responsible for it and will drop it when the
`Item` is dropped.

### 4. Why does `add_item` take `self` by `&mut` but `item` by value?

`&mut self` gives the method write access to the library so it can push into
`self.items`. `item` is taken by value because the library needs to own it — a
reference would not be enough, because the `Item` must live as long as the library
itself, not just as long as the caller's local scope.

### 5. When `add_item` returns `Err`, what happened to the `Item` the caller passed in? Was that a good design choice, and what is the alternative?

The `Item` is dropped — it was moved into the function and when `Err` is returned
the function ends, so Rust drops it. The caller loses access to it regardless.
This is a reasonable trade-off for a simple library: duplicates and empty titles
are programming errors, so discarding the item is fine. The alternative is to
return the item back to the caller inside the error variant
(e.g. `Err((LibraryError::EmptyTitle, item))`), which lets the caller recover and
reuse or correct it.

### 6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?

Returning `Option<Item>` would move the item out of the library's `Vec`, leaving
the library without it. The library must keep owning every item it stocks.
`Option<&Item>` lends a reference to the item for as long as the caller needs to
read it, while the library retains ownership throughout.

### 7. What is the lifetime `'a` in `items_by_author` actually saying?

```rust
pub fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item>
```

It says: the references inside the returned `Vec` live exactly as long as `self`
does. The caller cannot use the returned references after the library is dropped or
mutably borrowed, because those references point directly into the library's internal
`Vec`. The lifetime annotation makes this contract explicit and lets the compiler
enforce it.

### 8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same `Library` at once, and how did you structure the method around that?

Rust forbids two simultaneous mutable borrows into the same struct — taking
`&mut Item` from `self.items` and `&mut Member` from `self.members` at the same
time would require two `&mut self` borrows, which is not allowed. The method was
structured to do all validation with immutable borrows first, then release those
borrows, then do two separate mutable borrow passes: one to update the item's
status, and a second to update the member's list.

### 9. Why are `Library`'s fields private?

The library must keep an item's `LoanStatus` and a member's `borrowed_item_ids`
list in sync. If the fields were public, any code could update one without the
other, creating an inconsistent state (e.g. an item marked `OnLoan` but not
appearing in the member's list). Private fields force all mutations through the
library's methods, which always update both sides together.

### 10. What duplication does the provided `late_fee_cents` remove, and what would you lose by making it a free function instead?

The formula `overdue_days * daily_rate` would otherwise be written once in
`MediaKind`'s impl and again in `Item`'s impl. The default method on the trait
writes it once and both impls inherit it automatically. As a free function it would
still avoid duplication, but it would no longer be part of the trait — callers
would have to call it separately rather than calling `item.late_fee_cents(days)`,
and the trait would no longer guarantee every implementor gets the same formula.

### 11. Why is `Result` preferable to `panic!` for validation failures? Name a place in this crate where a panic would be defensible.

`Result` makes failures explicit in the type system and forces callers to decide
what to do — display an error, retry, log, or propagate. A `panic!` crashes the
thread with no chance to recover. Validation failures (unknown id, borrow limit,
empty title) are *expected* situations that callers should handle gracefully.

A place where panic is defensible: the internal `unwrap()` calls after validation
has already confirmed an item or member exists. At that point a `None` would mean
the library's own internal state is corrupt — a programming bug, not a user error —
and panicking is a reasonable response to that.

### 12. Which derive did you deliberately leave off a type, and why?

`Copy` was left off `Item`. `Item` contains a `String` (the title and author),
which allocates on the heap and cannot be bitwise-copied. Rust will not let you
derive `Copy` for a type that owns heap data — you can only derive it for types
whose fields are all `Copy`. Leaving it off forces the code to be explicit about
ownership: passing an `Item` is always a move, never a silent copy.

---

## Part 7 — Ownership compiler errors

### Experiment A — read `item.title` after `library.add_item(item)?`

```rust
let item = Item::new(1, "Dune".into(), "Frank Herbert".into(), MediaKind::Book { pages: 320 });
library.add_item(item).unwrap();
println!("{}", item.title); // ERROR
```

**Compiler output:**

```text
error[E0382]: borrow of moved value: `item`
 --> /tmp/exp_a.rs:6:20
  |
4 |     let item = Item::new(...);
  |         ---- move occurs because `item` has type `Item`, which does not implement the `Copy` trait
5 |     library.add_item(item).unwrap();
  |                      ---- value moved here
6 |     println!("{}", item.title);
  |                    ^^^^^^^^^^ value borrowed here after move
```

**Why:** `add_item` takes `item` by value, so ownership transfers into the library.
After the call, `item` is uninitialised from the caller's perspective. `Item` does
not implement `Copy` (it contains `String`), so the compiler cannot silently
duplicate it — the move is permanent.

---

### Experiment B — hold `find_item` result, then call `checkout`

```rust
let found = library.find_item(1);       // immutable borrow of library
library.checkout(1, 100, 0).unwrap();   // mutable borrow of library — conflict!
println!("{:?}", found);
```

**Compiler output:**

```text
error[E0502]: cannot borrow `library` as mutable because it is also borrowed as immutable
 --> /tmp/exp_b.rs:7:5
  |
6 |     let found = library.find_item(1);
  |                 ------- immutable borrow occurs here
7 |     library.checkout(1, 100, 0).unwrap();
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable borrow occurs here
8 |     println!("{:?}", found);
  |                      ----- immutable borrow later used here
```

**Why:** `find_item` returns a `&Item` that borrows from `library`. That borrow is
still alive (it is used in the `println!` on line 8). Meanwhile, `checkout` needs
`&mut library`. Rust forbids a mutable borrow while any immutable borrow is still
active — otherwise `checkout` could reallocate the internal `Vec`, invalidating the
reference `found` points to. The fix is to drop `found` before calling `checkout`,
or to not store it across the mutable call.

---

## Design notes

**Validate-then-mutate in `checkout`:** All validation is done with immutable
borrows first. Only after every check passes does the method re-enter the
collections with mutable borrows to update the item's status and the member's list.
This sidesteps the borrow checker issue from Experiment B and keeps the method
consistent — either everything succeeds or nothing changes.

**`late_fee_cents` via `saturating_sub`:** The overdue days calculation uses
`days_held.saturating_sub(loan_days())` which returns 0 instead of underflowing
when the item is returned on time. This means on-time returns automatically owe
nothing without a separate `if` branch.

**Ebooks never incur late fees:** `daily_late_fee_cents` returns 0 for `Ebook`.
Since the fee formula multiplies by that rate, the result is always 0 regardless
of how many days overdue — no special case needed in `return_item`.

---

## Example output

```text
=== Library stocked ===
  [1] "Dune" by Frank Herbert — Book (320 pages) — Available
  [2] "Project Hail Mary" by Andy Weir — Audiobook (540 minutes) — Available
  [3] "The Rust Programming Language" by Steve Klabnik — Ebook (1200 KB) — Available

Ada checks out item 1 on day 0.
  Item status: On loan to member 100 since day 0

Ada returns item 1 on day 30.
  Late fee owed: 225 cents (9 days overdue)
  Item status: Available

Handled error: Item with id 99 not found
```
