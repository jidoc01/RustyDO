use sha3::{Sha3_256, Digest};

/// We use SHA-3.
pub fn from_str(s: &str) -> Vec<u8> {
    let mut hasher = Sha3_256::default();
    hasher.update(s);
    hasher
        .finalize()
        .to_vec()
}