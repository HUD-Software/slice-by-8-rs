use crate::crc32::slice_by_8_with_seed;
use core::hash::{BuildHasher, BuildHasherDefault, Hasher};

/// Slice-By-8 hasher
#[derive(Debug, Default)]
pub struct CRC32Hasher {
    key: u32,
}

impl CRC32Hasher {
    /// Create a new [CRC32Hasher] initiated with a hash key
    pub fn with_seed(seed: u32) -> CRC32Hasher {
        CRC32Hasher { key: seed }
    }
}

impl Hasher for CRC32Hasher {
    /// Returns the hash value for the values written so far.
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::crc32::CRC32Hasher;
    /// use core::hash::Hasher;
    ///
    /// let hasher = CRC32Hasher::with_seed(0x4C2750BD);
    /// assert_eq!(hasher.finish(), 0x4C2750BD);
    /// ```
    fn finish(&self) -> u64 {
        self.key as u64
    }

    /// Writes some data into the [CRC32Hasher].
    ///
    /// # Example
    ///
    /// ```
    /// use slice_by_8::crc32::CRC32Hasher;
    /// use core::hash::Hasher;
    ///
    /// let mut hasher = CRC32Hasher::default();
    /// hasher.write(b"hash me!");
    /// ```
    fn write(&mut self, bytes: &[u8]) {
        self.key = slice_by_8_with_seed(bytes, self.key);
    }
}

impl BuildHasher for CRC32Hasher {
    type Hasher = CRC32Hasher;

    /// Creates a new [CRC32Hasher].
    fn build_hasher(&self) -> Self::Hasher {
        CRC32Hasher::default()
    }
}

/// A builder for default [CRC32Hasher].
pub type CRC32BuildHasher = BuildHasherDefault<CRC32Hasher>;

#[cfg(test)]
mod tests {

    use super::{CRC32BuildHasher, CRC32Hasher};
    use crate::crc32;
    use core::hash::{BuildHasher, Hasher};

    #[test]
    fn hasher_default() {
        let hasher = CRC32Hasher::default();
        assert_eq!(hasher.finish(), 0);
    }

    #[test]
    fn hasher_with_seed() {
        let hasher = CRC32Hasher::with_seed(0x9B9BEFFB);
        assert_eq!(hasher.finish(), 0x9B9BEFFB);
    }

    #[test]
    fn build_hasher() {
        let build_hasher = CRC32Hasher::default();
        let mut hasher = build_hasher.build_hasher();
        hasher.write(b"abcdefghijklmnopqrstuvwxyz");
        assert_eq!(hasher.finish(), 0x4C2750BD);
    }

    #[test]
    fn build_hasher_results_are_coherent_with_free_function() {
        let build_hasher = CRC32Hasher::default();
        let mut hasher = build_hasher.build_hasher();

        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

        // First hash is equivalent to city_hash_64 with no seed
        let hash_free_function = crc32::slice_by_8(HASH_ME);
        hasher.write(HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);

        // Second hash is equivalent to city_hash_64_with_seed with seed that is hash key of the first hash
        let hash_free_function = crc32::slice_by_8_with_seed(&HASH_ME, hash_free_function);
        hasher.write(&HASH_ME);
        assert_eq!(hasher.finish(), hash_free_function as u64);
    }

    #[test]
    fn hasher_is_usable_in_std_collections() {
        extern crate std;
        use std::collections::HashMap;
        const HASH_ME: &str = "hash me!";
        const VALUE: &str = "Hi";

        let mut map = HashMap::with_hasher(CRC32BuildHasher::default());
        map.insert(HASH_ME, VALUE);
        assert_eq!(map.get(&HASH_ME), Some(&VALUE));
    }
}
