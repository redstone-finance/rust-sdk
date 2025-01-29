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

The main documentation page [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/index.html)
lists the collection of utilities to make deserializing and decrypting RedStone payload as well as
the Rust based blockchain utils, cryptographic methods, network and contract primitives.

## Network

Network module contains primitives that allow to communicate via processor module with clear interface giving clear `Error` messages
and `Environment` trait all in one place [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/network/index.html).

## Configuration

Configuration module contains configuration for the RedStone payload processor [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/config/index.html).

- Configuration for the RedStone Rust-SDK is given as a pluggable entity. Any type implementing RedStoneConfig trait
  is a valid type to be used as config [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/trait.RedStoneConfig.html)
  Configuration requires to allow for perforimng two actions and to provide the Config type:
  - Crypto operations needed for address recovery. Different blockchains requires different cryptographic flavors.
  - Environment in which we execute. Provides logging etc. The same situation of flavors applies for the Environment.
  - Please check specific blockchain module for the implementation of Crypto and Environment modules:
    - Default [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/default_ext/index.html)
    - Solana [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/solana/index.html)
    - Radix [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/radix/index.html)
    - Casper [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/casper/index.html)
  - Config type - configuration for a RedStone payload processor.
    Specifies the parameters necessary for the verification and aggregation of values from various data points passed by the RedStone payload
    [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/config/struct.Config.html).

## Processor

Processor module contains the main processor of the RedStone payload [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/index.html).

- To process payload use `process_payload` function in processor module
  [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor/fn.process_payload.html) and its the main entrypoint in the core::processor module.
  It parses payload-bytes to DataPackages, consisting of DataPoints, see: https://docs.redstone.finance/img/payload.png
  each DataPackage is signed by a signer, to be recovered having the DataPackage's data and the signature (both are included in the payload-bytes).
  The recovering is important in the aggregation process: only the data signed by trusted signers (part of z Config) are counted to the aggregation (median value),
  it also validates timestamps (if they're not too old/far/future and all shall be equal).
  The aggregation (median values) is based on building a matrix: for each feed_id (in rows) from the Config the values for particular trusted signers in columns (taken by their indices).
  - The `config` argument is described above Configuration.
  - The `payload` argument is type that is convertible to bytes (Vec<u8>), and the bytes encoding is described in Redstone Finance Docs:
    [data formatting](https://docs.redstone.finance/docs/get-started/data-formatting-processing/#how-data-is-encoded-before-being-put-on-the-blockchain).
  - The result of `process_payload` [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor_result/type.ProcessorResult.html)
    returns success or an error:
    - Success contains a validated payload [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/core/processor_result/struct.ValidatedPayload.html)
      containing a `min_timestamp` in [ ms ] which is the minimum timestamp encountered during processing and `values` (`Vec<Value>`)
      where Value [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/struct.Value.html)
      Each element in this vector represents a processed value corresponding to the passed data_feed item in the Config

## Contract

Contract module contains contract primitives used for verification of the contract data and logic [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/contract/index.html)
For the specific verification primitive description look in to [here](https://docs.redstone.finance/rust/redstone/rust_sdk_2/redstone/contract/verification/index.html)
