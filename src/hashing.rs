use sha2::{Sha256, Digest};
use hex;

pub fn sha256_of(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    hex::encode(hasher.finalize())
}