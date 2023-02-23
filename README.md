<div align="center">
  <h1>Slice-by-8

  [![Crates.io](https://img.shields.io/crates/v/slice-by-8?logo=Docs.rs&style=flat-square)](https://crates.io/crates/slice-by-8) [![License](https://img.shields.io/crates/l/slice-by-8?style=flat-square)](https://choosealicense.com/licenses/mit/)
  </h1>
</div>

Rust improved implementation of the [Slice-by-8](http://slicing-by-8.sourceforge.net/) intel algorithm from the paper "*A Systematic Approach to building High Performance, Software-based, CRC Generators By Intel Researche and Development*"

Slice-by-8 do not load the standard library (a.k.a `#![no_std]`)

***Status***

[![Build](https://img.shields.io/github/actions/workflow/status/hud-software/slice-by-8-rs/Build.yml?label=Build&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/slice-by-8-rs/actions/workflows/Build.yml)
[![Clippy](https://img.shields.io/github/actions/workflow/status/hud-software/slice-by-8-rs/Clippy.yml?label=Clippy&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/slice-by-8-rs/actions/workflows/Clippy.yml)
[![docs.rs](https://img.shields.io/docsrs/slice-by-8/latest?label=Docs&logo=Docs.rs&logoColor=lightgrey&style=flat-square)](https://docs.rs/slice-by-8/1.0.4/slice_by_8/)
[![Test](https://img.shields.io/github/actions/workflow/status/hud-software/slice-by-8-rs/Test.yml?label=Tests&logo=Rust&logoColor=lightgrey&style=flat-square)](https://github.com/HUD-Software/slice-by-8-rs/actions/workflows/Test.yml)
[![codecov](https://img.shields.io/codecov/c/github/hud-software/slice-by-8-rs?label=Codecov&logo=Codecov&logoColor=lightgrey&style=flat-square&token=LTEI8LUT5R)](https://codecov.io/gh/HUD-Software/slice-by-8-rs)

**_Table of contents_**

1. [Introduction](#introduction)
2. [Usage](#usage)
    1. [Using Hasher](#using-hasher)
    2. [Using slice-by-8 functions](#using-slice-by-8-functions)
    3. [Using your own lookup table](#using-your-own_lookup_table)
3. [Generate Lookup table](#generate-lookup-table)
3. [Performance](#performance)

## Introduction

Slice-by-8 crate provides function that performs CRC hashing using improved variant of intel's [Slice-by-8](http://slicing-by-8.sourceforge.net/) algorithm.
The crate provides the slice-by-8 algorithm that take the loopup table to use as parameter if you want to use your own.
The crate also provides the CRC32 (Polynomial `0x04c11db7` ) available in `slice_by_8::crc32` and the CRC32c (Polynomial `0x1EDC6F41` ) in `slice_by_8::crc32c`.
CRC32c hash can use CRC32c intrinsics if enabled. You can enable intrinsic version on `x86_64` target_arch by enabling `sse4.2` target_feature or on `aarch64` target_arch by enabling `crc` target_feature.

## Usage

### Using Hasher

```rust
use slice_by_8::crc32::CRC32BuildHasher;
use std::collections::HashMap;
const KEY: &str = "hash";
const VALUE: &str = "me!";

// Create a HashMap that use CRC32Hasher to hash keys
let mut map = HashMap::with_hasher(CRC32BuildHasher::default());
map.insert(KEY, VALUE);

assert_eq!(map.get(&KEY), Some(&VALUE));
```

### Using slice-by-8 functions

Slice-by-8 provides functions to hash slice of bytes.

```rust
use slice_by_8::crc32c;

const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
assert_eq!(crc32c::slice_by_8(HASH_ME), 0x9EE6EF25);
```

***Note:*** `slice_by_8` is a similar to `slice_by_8_with_seed` with `seed` equals `0`.

### Using your own lookup table

You own lookup table must be `[[u32; 256]; 8]`.

```rust
use slice_by_8::slice_by_8;

let my_lookup_table: [[u32; 256]; 8] = slice_by_8::generate_table(slice_by_8::crc32::POLYNOMIAL);
const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

assert_eq!(slice_by_8(HASH_ME, &my_lookup_table), 0x4C2750BD);
```

## Generate Lookup table

The crate provide `generate_table` function to generate a lookup table from a polynomial.

```rust
use slice_by_8::generate_table;
use slice_by_8::{crc32, crc32c};

assert_eq!(generate_table(crc32::POLYNOMIAL), crc32::LOOKUP_TABLE);
assert_eq!(generate_table(crc32c::POLYNOMIAL), crc32c::LOOKUP_TABLE);
```

## Performance

Improvement are based on :

* [Stephan Brumme Fast CRC32](https://create.stephan-brumme.com/crc32/)
* [Redis CRC Speed Improvements](https://matt.sh/redis-crcspeed)
* [Unreal Engine 4](https://github.com/EpicGames/UnrealEngine/)
