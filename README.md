# Aqy Sdk

A rust Sdk for integrating with the [Aqy blockchain](https://docs.aqy.io/).

> [!NOTE]
> This is project is under development and many features may still be under
> development or missing.

## Overview

This repository contains a collection of libraries for integrating with the Aqy blockchain.

A few of the project's high-level goals are as follows:

* **Be modular** - user's should only need to pay the cost (in terms of dependencies/compilation time) for the features that they use.
* **Be light** - strive to have a minimal dependency footprint.
* **Support developers** - provide all needed types, abstractions and APIs to enable developers to build robust applications on Aqy.
* **Support wasm** - where possible, libraries should be usable in wasm environments.

## Crates

In an effort to be modular, functionality is split between a number of crates.

* [`aqy-sdk-types`](crates/aqy-sdk-types)
    [![aqy-sdk-types on crates.io](https://img.shields.io/crates/v/aqy-sdk-types)](https://crates.io/crates/aqy-sdk-types)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/aqy-sdk-types)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/aqy-rust-sdk/aqy_sdk_types/)
* [`aqy-crypto`](crates/aqy-crypto)
    [![aqy-crypto on crates.io](https://img.shields.io/crates/v/aqy-crypto)](https://crates.io/crates/aqy-crypto)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/aqy-crypto)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/aqy-rust-sdk/aqy_crypto/)
* [`aqy-graphql-client`](crates/aqy-graphql-client)
    [![aqy-graphql-client on crates.io](https://img.shields.io/crates/v/aqy-graphql-client)](https://crates.io/crates/aqy-graphql-client)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/aqy-graphql-client)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/aqy-rust-sdk/aqy-graphql-client/)

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
