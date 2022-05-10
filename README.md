# Slice-by-8 [![Crates.io](https://img.shields.io/crates/v/slice-by-8?style=plastic)](https://crates.io/crates/slice-by-8) [![Crates.io](https://img.shields.io/crates/l/slice-by-8?style=plastic)](https://choosealicense.com/licenses/mit/)

Rust improved implementation of the [Slice-by-8](http://slicing-by-8.sourceforge.net/) intel algorithm from the paper "*A Systematic Approach to building High Performance, Software-based, CRC Generators By Intel Researche and Development*"

Slice-by-8 do not load the standard library (a.k.a `#![no_std]`)

[![Build](https://github.com/HUD-Software/slice-by-8/actions/workflows/Build.yml/badge.svg)](https://github.com/HUD-Software/slice-by-8/actions/workflows/Build.yml) [![Test](https://github.com/HUD-Software/slice-by-8/actions/workflows/Test.yml/badge.svg)](https://github.com/HUD-Software/slice-by-8/actions/workflows/Test.yml) [![codecov](https://codecov.io/gh/HUD-Software/slice-by-8/branch/main/graph/badge.svg?token=KG7SEUBDUF)](https://codecov.io/gh/HUD-Software/slice-by-8) [![docs.rs](https://img.shields.io/docsrs/slice-by-8?style=plastic)](https://docs.rs/slice-by-8/latest/cityhash_sys/)

## Introduction

Slice-by-8 crate provides function that performs CRC32 and CRC32c hashing using improved variant of intel's [Slice-by-8](http://slicing-by-8.sourceforge.net/) algorithm.
The crate provides the slice-by-8 algorithm that take the loopup table to use as parameter if you want to use your own.
The crate also provides the CRC32 (Polynomial `0x04c11db7` ) available in `slice_by_8::crc32` and the CRC32c (Polynomial `0x1EDC6F41` ) in `slice_by_8::crc32c`.

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

const MY_LOOKUP_TABLE : [[u32; 256]; 8] = slice_by_8::generate_table(slice_by_8::crc32::POLYNOMIAL);
const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

assert_eq!(slice_by_8(HASH_ME, &MY_LOOKUP_TABLE), 0x4C2750BD);
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

## TODO

* [ ] ARM intrinsics version
* [ ] Intel intrinsics version
