# Infallible ToBigInt

[![Crates.io](https://img.shields.io/crates/v/{{project-name}}.svg)](https://crates.io/crates/{{project-name}})
[![Docs.rs](https://docs.rs/{{project-name}}/badge.svg)](https://docs.rs/{{project-name}})
[![CI](https://github.com/{{gh-username}}/{{project-name}}/workflows/CI/badge.svg)](https://github.com/{{gh-username}}/{{project-name}}/actions)

## Overview

This crate provides to traits `InfallibleToBigInt` and `InfallibleToBigUint`. They provide the `to_bigint(&self)` and
`to_biguint(&self)` from the popular [`num-bigint`](https://github.com/rust-num/num-bigint) crate. But they are only
implemented on types that support infallible conversion, and so they do not return a `Result` but only
`BigInt`/`BigUint`. This results in cleaner code.

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install infallible_tobigint`

## Usage

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
