
# This is a CLI app to get the ENS name and avatar for an address

Before running the app, you need to export the following environment variables:
## Export the RPC url 

```bash
export RPC_URL_MAINNET=https://your-rpc-url.com
```

## Export the ENS contract address

```bash
export ENS_CONTRACT_ADDRESS=0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e
```

## Running the app

```bash
RUST_LOG=info cargo run -- --address 0x0000000000000000000000000000000000000000
```
