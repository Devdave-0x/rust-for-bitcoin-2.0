# decodetrx — Bitcoin Transaction Decoder

Parses a raw Bitcoin transaction (hex) and prints a human-readable JSON summary.

## Usage

```
cargo run -- --tx <HEX>
```

## What it decodes

| Field | Description |
|---|---|
| `txid` | Double-SHA256 of the raw bytes, reversed (matches block explorers) |
| `version` | Transaction version (little-endian u32) |
| `inputs` | Each input: spending txid, output index, scriptSig, sequence |
| `outputs` | Each output: amount in BTC, scriptPubKey hex |
| `lock_time` | Earliest block/time this tx can be mined |
| `segwit` | Whether a SegWit marker was detected |

Amounts are serialized as BTC (satoshis ÷ 100,000,000).

## Examples

### Genesis block coinbase (non-SegWit)

```bash
cargo run -- --tx \
  01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73ffffffff0100f2052a01000000434104678afdb0fe5548271967f1a67130b7105cd6a828e03909a67962e0ea1f61deb649f6bc3f4cef38c4f35504e51ec112de5c384df7ba0b8d578a4c702b6bf11d5fac00000000
```

Output:
```json
{
  "txid": "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b",
  "version": 1,
  "inputs": [
    {
      "txid": "0000000000000000000000000000000000000000000000000000000000000000",
      "vout": 4294967295,
      "script_sig": "04ffff001d...",
      "sequence": 4294967295
    }
  ],
  "outputs": [
    {
      "amount": 50.0,
      "script_pubkey": "4104678afdb0..."
    }
  ],
  "lock_time": 0,
  "segwit": false
}
```

## Encoding rules

- All multi-byte integers use **little-endian** byte order.
- Variable-length fields are prefixed with a **CompactSize** (VarInt):
  - 1 byte for values 0–252
  - `0xfd` + 2 bytes for 253–65535
  - `0xfe` + 4 bytes for larger values
- SegWit transactions have a `0x00 0x01` marker+flag after the version field.
- Txid is the reversed double-SHA256 of the full raw bytes.
