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