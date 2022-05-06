#![doc=include_str!("../README.md")]
#![no_std]

mod crc32_hasher;
pub use crc32_hasher::*;

pub mod crc32;
pub mod crc32c;