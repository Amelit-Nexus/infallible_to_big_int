# Infallible ToBigInt

[![Crates.io](https://img.shields.io/crates/v/infallible_tobigint.svg)](https://crates.io/crates/infallible_tobigint)
[![Docs.rs](https://docs.rs/infallible_tobigint/badge.svg)](https://docs.rs/infallible_tobigint)
[![CI](https://github.com/Amelit-Nexus/infallible_tobigint/actions/workflows/rust.yml/badge.svg)](https://github.com/Amelit-Nexus/infallible_tobigint/actions)

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
```rust
use crate::infallible_to_big_int::*;

// use the conversion directly
153830.to_bigint();

// or define a function which takes any InfallibleToBigInt
fn do_great_things(to_bigint: impl InfallibleToBigInt) {
    let bigint = to_bigint.to_bigint();
    // ... do something nice with bigint here
}

// then you can call it like this
do_great_things(153830)
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
