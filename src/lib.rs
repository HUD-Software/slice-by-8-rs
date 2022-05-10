#![doc=include_str!("../README.md")]
#![no_std]
#![feature(stdsimd)]
#![feature(const_mut_refs)]
#![feature(const_maybe_uninit_assume_init)]

mod algorithm;
pub use algorithm::generate_table;
pub use algorithm::slice_by_8;
pub use algorithm::slice_by_8_with_seed;

pub mod crc32;
pub mod crc32c;
