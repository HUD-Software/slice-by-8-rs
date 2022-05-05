#![doc=include_str!("../README.md")]
#![no_std]

mod hasher;
mod lookup_table;
pub use hasher::SliceBy8Hasher;

mod crc32_hasher;
pub use crc32_hasher::SliceBy8BuildHasher;

mod crc32;
pub use crc32::*;
