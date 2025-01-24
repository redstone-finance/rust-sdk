# Rust-SDK - General Assumption

<!-- TOC -->

- [Rust-SDK General Assumption](#rust-sdk---general-assumption)
  - [Rust SDK Overview](#rust-sdk-overview)
  - [Network](#network)
  - [Configuration](#configuration)
  - [Processor](#processor)
  - [Contract](#contract)
  - [Cryptographic modules](#cryptographic-modules)

<!-- TOC -->

## Rust SDK Overview

The main documentation page [rust/redstone/rust_sdk_2/redstone/index.html](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/index.html)
lists the collection of utilities to make deserializing and decrypting RedStone payload as well as
the Rust based blockchain utils, cryptographic methods, network and contract primitives.

## Network

Network module contains primitives that allow to communicate via processor module with clear interface giving clear `Error` messages
and `Environment` trait all in one place [rust/redstone/rust_sdk_2/redstone/network](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/network/index.html).

## Configuration

Configuration module contains configuration for the RedStone payload processor [rust/redstone/rust_sdk_2/redstone/core/config](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/config/index.html).

- Configuration for the RedStone Rust-SDK is given as a pluggable entity. Any type implementing RedStoneConfig trait
  is a valid type to be used as config [redstone/trait.RedStoneConfig](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/trait.RedStoneConfig.html)
  Configuration requires to allow for perforimng two actions and to provide the Config type:
  - Crypto operations needed for address recovery. Different blockchains requires different cryptographic flavors.
    Please check specific blockchain module for the implementation:
    - Default [rust/redstone/rust_sdk_2/redstone/default_ext](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/default_ext/index.html)
    - Solana [rust/redstone/rust_sdk_2/redstone/solana](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/solana/index.html)
    - Radix [/rust/redstone/rust_sdk_2/redstone/radix](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/radix/index.html)
    - Casper [rust/redstone/rust_sdk_2/redstone/casper](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/casper/index.html)
  - Environment in which we execute. Provides logging etc. The same situation of flavors applies for the Environment.
    Please check specific blockchain module for the implementation:
    - Default [rust/redstone/rust_sdk_2/redstone/default_ext](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/default_ext/index.html)
    - Solana [rust/redstone/rust_sdk_2/redstone/solana](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/solana/index.html)
    - Radix [/rust/redstone/rust_sdk_2/redstone/radix](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/radix/index.html)
    - Casper [rust/redstone/rust_sdk_2/redstone/casper](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/casper/index.html)
  - Config type - configuration for a RedStone payload processor.
    Specifies the parameters necessary for the verification and aggregation of values from various data points passed by the RedStone payload.
    [rust/redstone/rust_sdk_2/redstone/core/config/struct.Config](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/config/struct.Config.html)

## Processor

Processor module contains the main processor of the RedStone payload [rust/redstone/rust_sdk_2/redstone/core/processor](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/index.html).

- To proses payload use `process_payload` function in processor module
  [rust/redstone/rust_sdk_2/redstone/core/processor/fn.process_payload.html](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/fn.process_payload.html)
  - The `config` argument is described above Configuration.
  - The `payload` argument is type that is convertible to bytes (Vec<u8>), and the bytes encoding is described in
    [redstone.finance docs data-formatting-processing](https://docs.redstone.finance/docs/get-started/data-formatting-processing/#how-data-is-encoded-before-being-put-on-the-blockchain)
  - The result of `process_payload` [rust/redstone/rust_sdk_2/redstone/core/processor_result/type.ProcessorResult](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor_result/type.ProcessorResult.html)
    returns success or an error:
    - Success contains a validated payload [rust/redstone/rust_sdk_2/redstone/core/processor_result/struct.ValidatedPayload](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor_result/struct.ValidatedPayload.html)
      containing a `min_timestamp` in [ ms ] which is the minimum timestamp encountered during processing and `values` (`Vec<Value>`)
      where [rust/redstone/rust_sdk_2/redstone/struct.Value](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/struct.Value.html)
      Each element in this vector represents a processed value corresponding to the passed data_feed item in the Config

## Contract

Contract module contains contract primitives used for verification of the contract data and logic [rust/redstone/rust_sdk_2/redstone/contract/index.html](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/contract/index.html)
For the specific verification primitive description look in to [rust/redstone/rust_sdk_2/redstone/contract/verification](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/contract/verification/index.html)

## Cryptographic methods

1. [`redstone` crate with
   `crypto_secp256k1`](https://docs.redstone.finance/rust/redstone/crypto_secp256k1/redstone/index.html)
   pure Rust implementation
2. [`redstone` crate with `crypto_secp256k1` and
   `network_casper`](https://docs.redstone.finance/rust/redstone/crypto_secp256k1,network_casper/redstone/index.html)
   extension for Casper
3. [`redstone` crate with `crypto_radix` and
   `network_radix`](https://docs.redstone.finance/rust/redstone/crypto_radix,network_radix/redstone/index.html)
   extension for Radix
