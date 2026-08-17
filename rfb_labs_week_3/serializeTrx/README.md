# serialize-trx

A command-line tool that constructs and serializes a Bitcoin transaction from
user-supplied arguments. No transaction data is hardcoded — every field is
provided at runtime.

## Build

```bash
cargo build --release
```

## Usage

```
serialize-trx [OPTIONS]

Options:
  -v, --version <VERSION>    Transaction version [default: 2]
      --segwit               Mark transaction as SegWit
      --locktime <LOCKTIME>  Locktime [default: 0]
  -i, --input <INPUT>        Input: <txid_hex>:<vout>:<sequence>[:<scriptsig_hex>]  (repeatable)
  -o, --output <OUTPUT>      Output: <value_sats>:<scriptpubkey_hex>  (repeatable)
  -w, --witness <WITNESS>    Witness items for one input: <hex>,<hex>,...  (one per --input, in order)
  -h, --help                 Print help
```

### Input format

```
<txid_hex>:<vout>:<sequence>[:<scriptsig_hex>]
```

- `txid_hex` — 32-byte (64 hex chars) previous transaction ID
- `vout` — previous output index (integer)
- `sequence` — sequence number (integer or `0xffffffff`)
- `scriptsig_hex` — optional; leave empty for SegWit inputs

### Output format

```
<value_sats>:<scriptpubkey_hex>
```

### Witness format

One `--witness` per input, in the same order as `--input`.
Comma-separated list of hex-encoded witness items:

```
--witness <item1_hex>,<item2_hex>
```

---

## Examples

### Simple non-SegWit transaction

```bash
cargo run -- \
  --version 1 \
  --input 8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:1:4294967295 \
  --output 69886:0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b \
  --output 29442:00149831122b93d21715c70db626ccc844d3c21f9687 \
  --locktime 0
```

### SegWit transaction with witness data

```bash
cargo run -- \
  --version 2 \
  --segwit \
  --input 8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:1:0xffffffff \
  --output 69886:0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b \
  --output 29442:00149831122b93d21715c70db626ccc844d3c21f9687 \
  --witness 3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301,029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358 \
  --locktime 0
```

### Multiple inputs and outputs

```bash
cargo run -- \
  --version 2 \
  --segwit \
  --input 8fb0d07bb3766421bff2d908b70e5de818e4d85a436ea3606310c1052b0dc821:0:0xffffffff \
  --input 1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef:1:0xffffffff \
  --output 50000:0014a632c1fff47af29f8c81dc4c6e91eb49a116c12b \
  --output 10000:00149831122b93d21715c70db626ccc844d3c21f9687 \
  --witness 3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301,029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358 \
  --witness 3045022100f8704a3e7d55d4b5ee448cc6365caeffa42c2b00f74a37726d4fa3c11982e3e502203591c4a4bde9200281755ae5a8759116ce6e0cc7f5d30cf0eeb5b2b74f74bab301,029cbb1e568de08f469a8751aa2000331f130ca92ad49012d9cececaf6f8eb2358 \
  --locktime 0
```

## Output

```
Serialized transaction (hex):
02000000000101...

Transaction size: 192 bytes
```

## Validation

The program validates:
- Hex strings are valid hex before conversion
- TXIDs are exactly 32 bytes (64 hex chars)
- At least one input and one output are provided
- Number of `--witness` entries matches number of `--input` entries
