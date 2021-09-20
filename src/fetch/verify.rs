use crate::error::Result;
use sha2::{ Digest, Sha256 };


/// Verifies some data based on the anchor part of an URL
pub struct Verify;
impl Verify {
    /// Verifies some data against a tag
    pub fn check<D, T>(data: D, tag: Option<T>) -> Result where D: AsRef<[u8]>, T: AsRef<str> {
        // Skip verification if there is no hash
        let tag = match tag.as_ref() {
            Some(tag) => tag.as_ref(),
            None => return Ok(())
        };

        // Verify the hash
        match tag.split_once("=") {
            Some(("sha256", hash)) => Self::check_sha256(data.as_ref(), hash),
            Some((algo, _)) => Err(einval!("Unsupported hash algorithm: {}", algo)),
            None => Err(einval!("Invalid resource hash"))
        }
    }

    /// Verifies a SHA2-256 hash
    fn check_sha256(data: &[u8], hash: &str) -> Result {
        match format!("{:x}", Sha256::digest(data)).as_str() {
            computed_hash if computed_hash == hash => Ok(()),
            computed_hash => Err(einval!("Hash mismatch; expected: {}, got: {}", hash, computed_hash))
        }
    }
}