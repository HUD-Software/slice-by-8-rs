# Slice-by-8 [![Crates.io](https://img.shields.io/crates/v/slice-by-8?style=plastic)](https://crates.io/crates/slice-by-8) [![Crates.io](https://img.shields.io/crates/l/slice-by-8?style=plastic)](https://choosealicense.com/licenses/mit/)

Rust improved implementation of the [Slice-by-8](http://slicing-by-8.sourceforge.net/) intel algorithm.  
Slice-by-8 do not load the standard library (a.k.a `#![no_std]`)

[![Build](https://github.com/HUD-Software/slice-by-8/actions/workflows/Build.yml/badge.svg)](https://github.com/HUD-Software/slice-by-8/actions/workflows/Build.yml) 
[![Test](https://github.com/HUD-Software/slice-by-8/actions/workflows/Test.yml/badge.svg)](https://github.com/HUD-Software/slice-by-8/actions/workflows/Test.yml)
[![codecov](https://codecov.io/gh/HUD-Software/slice-by-8/branch/main/graph/badge.svg?token=KG7SEUBDUF)](https://codecov.io/gh/HUD-Software/slice-by-8) [![docs.rs](https://img.shields.io/docsrs/slice-by-8?style=plastic)](https://docs.rs/slice-by-8/latest/cityhash_sys/)

## Introduction

Slice-by-8 provides hash function that performs CRC32 hashing using improved variant of intel's [Slice-by-8](http://slicing-by-8.sourceforge.net/) algorithm.
Polynomial used in lookup table is 0x04c11db7.

Slice-by-8 is tested on little-endian but should work on big-endian architecture.

## Usage

### Using Hasher
```rust
use slice_by_8::crc32::SliceBy8BuildHasher;
use std::collections::HashMap;
const KEY: &str = "hash";
const VALUE: &str = "me!";

// Create a HashMap that use SliceBy8Hasher to hash keys
let mut map = HashMap::with_hasher(SliceBy8BuildHasher::default());
map.insert(KEY, VALUE);

assert_eq!(map.get(&KEY), Some(&VALUE));
```

### Using slice-by-8 functions
Slice-by-8 provides functions to hash slice of bytes.

```rust ignore
fn slice_by_8(buf: &[u8]) -> u32;
fn slice_by_8_with_seed(buf: &[u8], seed: u32) -> u32;
```
**_Note_**`slice_by_8` is a similar to `slice_by_8_with_seed` with `seed` equals `0`.

## Performance

This implementation is an improvement of the intel algorithm.
Improvement are based on :
* [Stephan Brumme Fast CRC32](https://create.stephan-brumme.com/crc32/)
* [Redis CRC Speed Improvements](https://matt.sh/redis-crcspeed)
* [Unreal Engine 4](https://github.com/EpicGames/UnrealEngine/)

## TODO
- [ ] Select CRC32 or CRC32C version (https://crccalc.com/)
- [ ] Lookup table value tested
- [ ] ARM intrinsics version
- [ ] Intel intrinsics version   