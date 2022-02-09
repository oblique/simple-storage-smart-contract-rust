Example of a simple Ethereum smart contract that interacts with Rust.

# How to run

Start a mainnet fork:

```
ganache-cli -d
```

Run it:

```
export WALLET_PRIV_KEY=0x4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d
export NETWORK_URL=http://127.0.0.1:8545
cargo run
```
