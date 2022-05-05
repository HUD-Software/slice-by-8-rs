#![no_std]
//!
//! Rust implementation of the CRC32c [Slice-by-8](http://slicing-by-8.sourceforge.net/) intel algorithm.
//! It provides an improved version of the original intel slice-by-8. 
//! 
//! Slice-by-8 do not load the standard library (a.k.a `#![no_std]`)
//! 
//! See the README for more informations
//! 
//! # Example
//! ```rust
//! use slice_by_8::SliceBy8BuildHasher;
//! use std::collections::HashMap;
//! const KEY: &str = "hash";
//! const VALUE: &str = "me!";
//! 
//! // Create a HashMap that use SliceBy8Hasher to hash keys
//! let mut map = HashMap::with_hasher(SliceBy8BuildHasher::default());
//! map.insert(KEY, VALUE);
//! 
//! assert_eq!(map.get(&KEY), Some(&VALUE));
//! ```
//! 
mod lookup_table;
mod hasher;
pub use hasher::*;
mod slice_by_8;
pub use crate::slice_by_8::*;



