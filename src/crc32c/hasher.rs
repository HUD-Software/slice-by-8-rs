use crate::crc32c::slice_by_8_with_seed;
use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

/// Slice-By-8 hasher
#[derive(Debug, Default)]
pub struct CRC32CHasher {
    key: u32,
}

impl CRC32CHasher {
    /// Create a new [CRC32CHasher] initiated with a hash key
    pub fn with_seed(seed: u32) -> CRC32CHasher {
        CRC32CHasher { key: seed }
    }
}

impl Hasher for CRC32CHasher {
    /// Returns the hash value for the values written so far.
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::crc32c::CRC32CHasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CRC32CHasher::with_seed(0x4C2750BD);
    /// assert_eq!(hasher.finish(), 0x4C2750BD);
    /// ```
    fn finish(&self) -> u64 {
        self.key as u64
    }

    /// Writes some data into the [CRC32CHasher].
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::crc32c::CRC32CHasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = CRC32CHasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = slice_by_8_with_seed(bytes, self.key);
    }
}

impl BuildHasher for CRC32CHasher {
    type Hasher = CRC32CHasher;

    /// Creates a new [CRC32CHasher].
    fn build_hasher(&self) -> Self::Hasher {
        CRC32CHasher::default()
    }
}

/// A builder for default [CRC32CHasher].
pub type CRC32CBuildHasher = BuildHasherDefault<CRC32CHasher>;

#[cfg(test)]
mod tests {

    use super::{CRC32CBuildHasher, CRC32CHasher};
    use crate::crc32c;
    use core::hash::{BuildHasher, Hasher};

    #[test]
    fn hasher_default() {
        let hasher = CRC32CHasher::default();
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn hasher_with_seed() {
        let hasher = CRC32CHasher::with_seed(0x9B9BEFFB);
        assert_eq!(hasher.finish(), 0x9B9BEFFB);
    }

    #[test]
    fn build_hasher() {
        let build_hasher = CRC32CHasher::default();
        let mut hasher = build_hasher.build_hasher();
        hasher.write(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(hasher.finish(), 0x9EE6EF25);
    }

    #[test]
    fn build_hasher_results_are_coherent_with_free_function() {
        let build_hasher = CRC32CHasher::default();
        let mut hasher = build_hasher.build_hasher();

        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        // First hash is equivalent to city_hash_64 with no seed
        let hash_free_function = crc32c::slice_by_8(HASH_ME);
        hasher.write(HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);

        // Second hash is equivalent to city_hash_64_with_seed with seed that is hash key of the first hash
        let hash_free_function = crc32c::slice_by_8_with_seed(HASH_ME, hash_free_function);
        hasher.write(HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);
    }

    #[test]
    fn hasher_is_usable_in_std_collections() {
        extern crate std;
        use std::collections::HashMap;
        const HASH_ME: &str = "hash me!";
        const VALUE: &str = "Hi";

        let mut map = HashMap::with_hasher(CRC32CBuildHasher::default());
        map.insert(HASH_ME, VALUE);
        assert_eq!(map.get(&HASH_ME), Some(&VALUE));
    }
}
