# Slice-by-8

Rust implementation of the [Slice-by-8](http://slicing-by-8.sourceforge.net/) intel algorithm.
It provides an improved version of the original intel slice-by-8.
Slice-by-8 do not load the standard library (a.k.a `#![no_std]`)

## Introduction

Slice-by-8 provides hash function that performs CRC32c hashing using improved variant of intel's [Slice-by-8](http://slicing-by-8.sourceforge.net/) algorithm.
Slice-by-8 is tested on little-endian but should work on big-endian architecture.

## Usage

### Using Hasher
```rust
use slice_by_8::SliceBy8BuildHasher;
use std::collections::HashMap;
const KEY: &str = "hash";
const VALUE: &str = "me!";

// Create a HashMap that use CityHash64 to hash keys
let mut map = HashMap::with_hasher(SliceBy8BuildHasher::default());
map.insert(KEY, VALUE);

assert_eq!(map.get(&KEY), Some(&VALUE));
```

### Using Portable CityHash functions
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
- [ ] Lookup table value tested
- [ ] ARM intrinsics version
- [ ] Intel intrinsics version   