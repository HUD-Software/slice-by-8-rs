use core::hash::{BuildHasher, BuildHasherDefault, Hasher};
use crate::slice_by_8_with_seed;

/// Slice-By-8 hasher
#[derive(Debug, Default)]
pub struct SliceBy8Hasher {
    key: u32,
}


impl SliceBy8Hasher {
    /// Create a new [SliceBy8Hasher] initiated with a hash key
    pub fn with_seed(seed: u32) -> SliceBy8Hasher {
        SliceBy8Hasher { key: seed }
    }
}


impl Hasher for SliceBy8Hasher {
    /// Returns the hash value for the values written so far.
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::SliceBy8Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = SliceBy8Hasher::with_seed(0x4C2750BD);
    /// assert_eq!(hasher.finish(), 0x4C2750BD);
    /// ```
    fn finish(&self) -> u64 {
        self.key as u64
    }

    /// Writes some data into the [SliceBy8Hasher].
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::SliceBy8Hasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = SliceBy8Hasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = slice_by_8_with_seed(bytes, self.key);
    }
}

impl BuildHasher for SliceBy8Hasher {
    type Hasher = SliceBy8Hasher;

    /// Creates a new [SliceBy8Hasher].
    fn build_hasher(&self) -> Self::Hasher {
        SliceBy8Hasher::default()
    }
}

/// A builder for default [SliceBy8Hasher].
pub type SliceBy8BuildHasher = BuildHasherDefault<SliceBy8Hasher>;


#[cfg(test)]
mod tests {

    use core::hash::{Hasher, BuildHasher};
    use crate::slice_by_8;
    use super::{SliceBy8Hasher, SliceBy8BuildHasher};

    #[test]
    fn hasher_default() {
        let hasher = SliceBy8Hasher::default();
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn hasher_with_seed() {
        let hasher = SliceBy8Hasher::with_seed(0x9B9BEFFB);
        assert_eq!(hasher.finish(), 0x9B9BEFFB);
    }

    #[test]
    fn build_hasher() {
        let build_hasher = SliceBy8BuildHasher::default();
        let mut hasher = build_hasher.build_hasher();
        hasher.write(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(hasher.finish(), 0x4C2750BD);
    }

    #[test]
    fn build_hasher_results_are_coherent_with_free_function() {
        let build_hasher = SliceBy8BuildHasher::default();
        let mut hasher = build_hasher.build_hasher();

        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        // First hash is equivalent to city_hash_64 with no seed
        let hash_free_function = slice_by_8::slice_by_8(HASH_ME);
        hasher.write(HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);

        // Second hash is equivalent to city_hash_64_with_seed with seed that is hash key of the first hash
        let hash_free_function = slice_by_8::slice_by_8_with_seed(&HASH_ME, hash_free_function);
        hasher.write(&HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);
    }

        #[test]
        fn hasher_is_usable_in_std_collections() {
            extern crate std;
            use std::collections::HashMap;
            const HASH_ME: &str = "hash me!";
            const VALUE: &str = "Hi";

            let mut map = HashMap::with_hasher(SliceBy8BuildHasher::default());
            map.insert(HASH_ME, VALUE);
            assert_eq!(map.get(&HASH_ME), Some(&VALUE));
        }
}