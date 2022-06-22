#![doc=include_str!("../README.md")]
#![no_std]
#![cfg_attr(all(target_arch = "aarch64", target_feature = "crc"), feature(stdsimd))]

mod algorithm;
pub use algorithm::generate_table;
pub use algorithm::slice_by_8;
pub use algorithm::slice_by_8_with_seed;

pub mod crc32;
pub mod crc32c;
