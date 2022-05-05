use crate::lookup_table;

/// Computes the CRC32 checksum for the specified buffer using the slicing by 8
/// algorithm over 64 bit quantities.
///
/// # Example
/// ```
/// const HASH_ME : &[u8] = b"abcdefghijklmnopqrstuvwxyz";
/// assert_eq!(slice_by_8::slice_by_8(HASH_ME), 0x4C2750BD);
/// ```
pub fn slice_by_8(buf: &[u8]) -> u32 {
    slice_by_8_with_seed(buf, 0)
}

/// Computes the CRC32 checksum for the specified buffer using the slicing by 8
/// algorithm over 64 bit quantities, adding a seed to the result.
///
/// # Example
/// ```
/// const HASH_ME : &[u8] = b"abcdefghijklmnopqrstuvwxyz";
/// assert_eq!(slice_by_8::slice_by_8_with_seed(HASH_ME, 123456789), 0xEADB5034);
/// ```
pub fn slice_by_8_with_seed(buf: &[u8], seed: u32) -> u32 {
    // Based on the Slicing-by-8 intel algorithm : http://slicing-by-8.sourceforge.net/
    // Alignement optimisation come from https://matt.sh/redis-crcspeed
    let mut crc = !seed;

    // Consume all bits until we are 8 bits aligned
    let (prefix, shorts, suffix) = unsafe { buf.align_to::<u64>() };
    crc = prefix.iter().fold(crc, |acc, byte| {
        lookup_table::CRC32_LOOKUP[0][((acc ^ *byte as u32) & 0xff) as usize] ^ (acc >> 8)
    });

    // Process eight bytes at once (Slicing-by-8)
    crc = shorts.iter().fold(crc, |acc, bytes| {
        if cfg!(target_endian = "big") {
            // Maybe inverse?
            let (low, high) = (
                (*bytes as u32) ^ acc.reverse_bits(),
                (*bytes >> u32::BITS) as u32,
            );
            lookup_table::CRC32_LOOKUP[0][(high & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[1][((high >> 8) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[2][((high >> 16) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[3][((high >> 24) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[4][(low & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[5][((low >> 8) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[6][((low >> 16) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[7][((low >> 24) & 0xFF) as usize]
        }
        // cfg!(target_endian = "little")
        else {
            let (low, high) = ((*bytes as u32) ^ acc, (*bytes >> u32::BITS) as u32);
            lookup_table::CRC32_LOOKUP[0][((high >> 24) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[1][((high >> 16) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[2][((high >> 8) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[3][(high & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[4][((low >> 24) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[5][((low >> 16) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[6][((low >> 8) & 0xFF) as usize]
                ^ lookup_table::CRC32_LOOKUP[7][(low & 0xFF) as usize]
        }
    });

    // Consume remaining 1 to 7 bytes (standard algorithm)
    !suffix.iter().fold(crc, |acc, byte| {
        (acc >> 8) ^ lookup_table::CRC32_LOOKUP[0][((acc ^ *byte as u32) & 0xff) as usize]
    })
}
#[cfg(test)]
mod tests {

    #[test]
    fn slice_by_8_no_seed() {
        // Miss align to be sure we handle this case
        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        assert_eq!(super::slice_by_8(HASH_ME), 0x4C2750BD);
    }
    #[test]
    fn slice_by_8_with_seed() {
        // Miss align to be sure we handle this case
        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        assert_eq!(super::slice_by_8_with_seed(HASH_ME, 123456789), 0xEADB5034);
    }
}
