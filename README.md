# Foucoco-Standalone

This is standalone version of Foucoco testnet. It uses the `manual-seal` consensus algorithm
so that blocks are created and immediately whenever a transaction is submitted.

This is particularly useful for running test scripts, e.g., for smart contracts.

This repository is based on the simple [substrate-contracts-node](https://github.com/paritytech/substrate-contracts-node) project.

## How to run

Just execute `cargo run --release`
