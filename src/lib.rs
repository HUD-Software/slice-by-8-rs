#![doc=include_str!("../README.md")]
#![no_std]

mod algorithm;
pub use algorithm::slice_by_8;
pub use algorithm::slice_by_8_with_seed;

pub mod crc32;
pub mod crc32c;
