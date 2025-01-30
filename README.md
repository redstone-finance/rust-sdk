# RustSDK - Audit guide

This repository is an integral part of the https://github.com/redstone-finance/redstone-oracles-monorepo repository,
especially of the solana-connector package (https://github.com/redstone-finance/redstone-oracles-monorepo/tree/main/packages/solana-connector) and is subject of all their licenses.

<!-- TOC -->

- [RustSDK - Audit guide](#rustsdk---audit-guide)
  - [Repository](#repository)
  - [Code description](#code-description)
    - [Documentation](#documentation)
    - [Rust APIs and SDKs code conventions](#rust-apis-and-sdks-code-conventions)
  - [What should be audited](#what-should-be-audited)
    - [Rust files](#rust-files)
    - [Other files](#other-files)

<!-- TOC -->

## Repository

- The repository: https://github.com/redstone-finance/rust-sdk
- The CommitId: [will be prepared for the particular version]
- Path: [crates/redstone](./src)

The direct path should look like:
[https://github.com/redstone-finance/rust-sdk/tree/[COMMIT_ID]/crates/redstone](https://github.com/redstone-finance/rust-sdk/tree/main/crates/redstone)

## Code description

### Documentation

- Redstone crate documentation [here](./crates/redstone/README.md)
  - RedstoneConfig is abstraction required as configuration for the RedStone protocol
    [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/trait.RedStoneConfig.html).
  - Process Payload function is the main processor of the RedStone payload
    [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/fn.process_payload.html).
- General [RedStone Blockchain Oracles docs](https://docs.redstone.finance/docs/get-started/data-formatting-processing/)
  - Especially [The push model docs](https://docs.redstone.finance/docs/get-started/models/redstone-push/)

### Rust APIs and SDKs code conventions

We try to follow conventions from [here](https://rust-lang.github.io/api-guidelines/checklist.html).

## What should be audited

### Rust files

## The code-lines in `*/crates/redstone/src/**/*.rs` files should be audited besides files listed below:

- ./crates/redstone/src/core/test_helpers.rs - file should not be audited.
- ./crates/redstone/src/helpers - directory should not be audited.
- all test, imports and comments should not be included for audit.

### Other files

- We suggest to read `**/README.md`
- We suggest to audit Dependencies inside `**/Cargo.toml` files
