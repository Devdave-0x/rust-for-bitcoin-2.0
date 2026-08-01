# Lab 02 — Wallets and addresses

## Commands used

```bash
# Create the miner wallet
bitcoin-cli createwallet "miner"

# Create the receiver wallet
bitcoin-cli createwallet "receiver"

# List loaded wallets to confirm both are present
bitcoin-cli listwallets

# Generate the mining address (label: mining) inside the miner wallet
bitcoin-cli -rpcwallet=miner getnewaddress "mining"

# Generate the classmate address inside the receiver wallet
bitcoin-cli -rpcwallet=receiver getnewaddress "classmate"

# Verify the mining address belongs to the miner wallet
bitcoin-cli -rpcwallet=miner getaddressinfo "<mining-address>"

# Verify the classmate address belongs to the receiver wallet
bitcoin-cli -rpcwallet=receiver getaddressinfo "<classmate-address>"
```

## Terminal output

```
$ bitcoin-cli createwallet "miner"
{ "name": "miner" }

$ bitcoin-cli createwallet "receiver"
{ "name": "receiver" }

$ bitcoin-cli listwallets
[ "", "miner", "receiver" ]

$ bitcoin-cli -rpcwallet=miner getnewaddress "mining"
bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6

$ bitcoin-cli -rpcwallet=receiver getnewaddress "classmate"
bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz

$ bitcoin-cli -rpcwallet=miner getaddressinfo bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6
{
  "address": "bcrt1q026m02sp292s2wlu8dkdkeq7c0mfd6gcs2auw6",
  "scriptPubKey": "00147ab5b7aa015155053bfc3b6cdb641ec3f696e918",
  "ismine": true,
  "labels": [ "mining" ]
}

$ bitcoin-cli -rpcwallet=receiver getaddressinfo bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz
{
  "address": "bcrt1qxz49w5y0ndd97efscpny5xcqyxq9zfrn8t72yz",
  "scriptPubKey": "001430aa57508f9b5a5f6530c0664a1b002180512473",
  "ismine": true,
  "labels": [ "classmate" ]
}
```

## Evidence references

TODO: Screenshot showing the Polar node, both wallets listed, and the
getaddressinfo output for each address. Name it evidence/lab02_wallets.png.

## Explanation

Bitcoin Core can manage multiple wallets simultaneously. When you call an RPC
that is wallet-specific — such as `getnewaddress`, `getbalances`, or
`sendtoaddress` — the node needs to know which wallet to use. The
`-rpcwallet=<name>` flag tells `bitcoin-cli` to include the wallet name in the
HTTP path of the RPC request (`/wallet/<name>`), directing the call to the
correct wallet context.

Without `-rpcwallet`, a wallet-scoped call either fails with an error (if more
than one wallet is loaded) or silently operates on the wrong wallet. For
example, calling `getnewaddress` without specifying the wallet when both
`miner` and `receiver` are loaded would produce an ambiguity error. Specifying
the wrong wallet — e.g. querying the miner wallet for an address that belongs
to the receiver wallet — causes `getaddressinfo` to return `"ismine": false`,
proving that addresses are scoped to the wallet that generated them and that
wallet context is essential for correct results.
