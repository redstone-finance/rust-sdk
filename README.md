# RustSDK - Audit guide

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

- Redstone crate documentation [./crates/redstone/README.md](./crates/redstone/README.md)
  - RedstoneConfig is abstraction required as configuration for the RedStone protocol.
    [redstone/trait.RedStoneConfig](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/trait.RedStoneConfig.html)
  - Process Payload function is the main processor of the RedStone payload
    [redstone/core/processor/fn.process_payload](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/fn.process_payload.html)
- General [RedStone Blockchain Oracles docs](https://docs.redstone.finance/docs/get-started/data-formatting-processing/)
  - Especially [The push model docs](https://docs.redstone.finance/docs/get-started/models/redstone-push/)

### Rust APIs and SDKs code conventions

We try to follow conventions from [here](https://rust-lang.github.io/api-guidelines/checklist.html).

## What should be audited

### Rust files

## The code-lines in `*/crates/redstone/src/**/*.rs` files should be audited besides files listed below:

- ./crates/redstone/src/core/test_helpers.rs - file should not be audited.
- ./crates/redstone/src/helpers - directory should not be audited.

### Other files

- We suggest to read `**/README.md`
- We suggest to audit Dependencies inside `**/Cargo.toml` files
