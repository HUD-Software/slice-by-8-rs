//!
//! This CRC implementation is based on slice-by-8 intel algorithm from the paper
//! "A Systematic Approach to building High Performance, Software-based, CRC Generators"
//! By Intel Researche and Development"
//! Adapation from <https://create.stephan-brumme.com/crc32/>
//! LookUpTable generated with polynomial 0x04c11db7

use core::mem::MaybeUninit;

/// Computes the CRC checksum for the specified buffer using the slicing by 8
/// algorithm over 64 bit quantities.
///
/// # Example
/// ```ignore
/// const MY_LOOKUP_TABLE : &[[u32; 256]; 8] = [[...],[...],...];
/// const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
///
/// assert_eq!(slice_by_8(HASH_ME, &MY_LOOKUP_TABLE), 0x...);
/// ```
#[inline(always)]
pub fn slice_by_8(buf: &[u8], lookup_table: &[[u32; 256]; 8]) -> u32 {
    slice_by_8_with_seed(buf, 0, lookup_table)
}

/// Computes the CRC checksum for the specified buffer using the slicing by 8
/// algorithm over 64 bit quantities, adding a seed to the result.
///
/// # Example
/// ```ignore
/// const MY_LOOKUP_TABLE : &[[u32; 256]; 8] = [[...],[...],...];
/// const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
///
/// assert_eq!(slice_by_8_with_seed(HASH_ME, 123456789, &MY_LOOKUP_TABLE), 0x...);
/// ```
pub fn slice_by_8_with_seed(buf: &[u8], seed: u32, lookup_table: &[[u32; 256]; 8]) -> u32 {
    let mut crc = !seed;

    // Consume all bits until we are 8 bits aligned
    let (prefix, shorts, suffix) = unsafe { buf.align_to::<u64>() };
    crc = prefix.iter().fold(crc, |acc, byte| {
        lookup_table[0][((acc ^ *byte as u32) & 0xff) as usize] ^ (acc >> 8)
    });

    // Process eight bytes at once (Slicing-by-8)
    #[cfg(target_endian = "big")]
    let process_8_bytes_at_once = |acc:u32, byte:&u64| {
        let byte = *byte;
        let (low, high) = (
            (byte as u32) ^ acc.reverse_bits(),
            (byte >> u32::BITS) as u32,
        );
        lookup_table[0][(high & 0xFF) as usize]
            ^ lookup_table[1][((high >> 8) & 0xFF) as usize]
            ^ lookup_table[2][((high >> 16) & 0xFF) as usize]
            ^ lookup_table[3][((high >> 24) & 0xFF) as usize]
            ^ lookup_table[4][(low & 0xFF) as usize]
            ^ lookup_table[5][((low >> 8) & 0xFF) as usize]
            ^ lookup_table[6][((low >> 16) & 0xFF) as usize]
            ^ lookup_table[7][((low >> 24) & 0xFF) as usize]
    };

    #[cfg(target_endian = "little")]
    let process_8_bytes_at_once = |acc:u32, byte:&u64| {
        let byte = *byte;
        let (low, high) = ((byte as u32) ^ acc, (byte >> u32::BITS) as u32);
            lookup_table[0][((high >> 24) & 0xFF) as usize]
                ^ lookup_table[1][((high >> 16) & 0xFF) as usize]
                ^ lookup_table[2][((high >> 8) & 0xFF) as usize]
                ^ lookup_table[3][(high & 0xFF) as usize]
                ^ lookup_table[4][((low >> 24) & 0xFF) as usize]
                ^ lookup_table[5][((low >> 16) & 0xFF) as usize]
                ^ lookup_table[6][((low >> 8) & 0xFF) as usize]
                ^ lookup_table[7][(low & 0xFF) as usize]
    };
    crc = shorts.iter().fold(crc, process_8_bytes_at_once);

    // Consume remaining 1 to 7 bytes (standard algorithm)
    !suffix.iter().fold(crc, |acc, byte| {
        (acc >> 8) ^ lookup_table[0][((acc ^ *byte as u32) & 0xff) as usize]
    })
}

/// Generate a lookup table.
/// The given polynomial is reversed before the generation
/// 
/// # Example
/// ```
/// use slice_by_8::{crc32,generate_table};
/// 
/// assert_eq!(generate_table(crc32::POLYNOMIAL), crc32::CRC32_LOOKUP_TABLE);
/// ```
pub fn generate_table(polynomial: u32) -> [[u32;256];8] {
    let mut generated_lookup_table= MaybeUninit::<[[u32; 256]; 8]>::uninit();

    unsafe {
        for i in 0..=0xFF {
            let mut crc: u32 = i;
            for _ in 0..8 {
                crc = (crc >> 1) ^ ((crc & 1) * polynomial.reverse_bits());
            }
            generated_lookup_table.assume_init_mut()[0][i as usize] = crc;
        }

        for i in 0..=0xFF {
            generated_lookup_table.assume_init_mut()[1][i] = (generated_lookup_table.assume_init()[0][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[0][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[2][i] = (generated_lookup_table.assume_init()[1][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[1][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[3][i] = (generated_lookup_table.assume_init()[2][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[2][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[4][i] = (generated_lookup_table.assume_init()[3][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[3][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[5][i] = (generated_lookup_table.assume_init()[4][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[4][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[6][i] = (generated_lookup_table.assume_init()[5][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[5][i] & 0xFF) as usize];
            generated_lookup_table.assume_init_mut()[7][i] = (generated_lookup_table.assume_init()[6][i] >> 8) ^ generated_lookup_table.assume_init()[0][(generated_lookup_table.assume_init()[6][i] & 0xFF) as usize];
        }
        *generated_lookup_table.assume_init_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate as slice_by_8;
    use crate::crc32::CRC32_LOOKUP_TABLE;

    #[test]
    fn slice_by_8_no_seed() {
        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        assert_eq!(slice_by_8::slice_by_8(HASH_ME, &CRC32_LOOKUP_TABLE), 0x4C2750BD);
    }

    #[test]
    fn slice_by_8_with_seed() {
        const HASH_ME: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
        assert_eq!(slice_by_8::slice_by_8_with_seed(HASH_ME, 123456789, &CRC32_LOOKUP_TABLE), 0xEADB5034);
    }
}