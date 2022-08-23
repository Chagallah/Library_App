# Library App smart contract rust

## About

This is a library_app project to automate the library services meant for a library with many users making it easier to accommodate each user's needs and locating books with ease.

### Initialization for dependencies needed

`use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};`

- Write your smart contract in `src\lib.rs`
- Test the contract

     `cargo test -- --nocapture`

- Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`

Built with the Near Rust Template    

**Get more info at:**

* [Rust Smart Contract Quick Start](https://docs.near.org/develop/prerequisites)
* [Rust SDK Book](https://www.near-sdk.io/)