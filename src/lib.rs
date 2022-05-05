#![doc=include_str!("../README.md")]
#![no_std]

mod lookup_table;
mod hasher;
pub use hasher::*;
mod slice_by_8;
pub use crate::slice_by_8::*;



