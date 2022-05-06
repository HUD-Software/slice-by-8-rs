#![doc=include_str!("../README.md")]
#![no_std]

mod lookup_table;

mod crc32_hasher;
pub use crc32_hasher::*;

mod crc32;
pub use crc32::*;