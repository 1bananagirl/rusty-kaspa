use crate::hex::FromHex;
use std::fmt::Display;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// generates by `build.rs`
const FULL_HASH: &str = env!("RUSTY_KASPA_GIT_FULL_COMMIT_HASH");
const SHORT_HASH: &str = env!("RUSTY_KASPA_GIT_SHORT_COMMIT_HASH");

/// Check if the codebase is built under a Git repository
/// and return the hash of the current commit as `Vec<u8>`.
pub fn hash() -> Option<Vec<u8>> {
    FromHex::from_hex(FULL_HASH).ok()
}

pub fn short_hash() -> Option<Vec<u8>> {
    FromHex::from_hex(SHORT_HASH).ok()
}

pub fn hash_str() -> Option<&'static str> {
    #[allow(clippy::const_is_empty)]
    (!FULL_HASH.is_empty()).then_some(FULL_HASH)
}

pub fn short_hash_str() -> Option<&'static str> {
    #[allow(clippy::const_is_empty)]
    (!SHORT_HASH.is_empty()).then_some(SHORT_HASH)
}

pub fn version() -> String {
    format!("v{VERSION}-{SHORT_HASH}")
}

pub fn with_short_hash<V>(version: V) -> impl Display
where
    V: Display,
{
    if let Some(short_hash) = short_hash_str() {
        format!("{version}-{short_hash}")
    } else {
        version.to_string()
    }
}

#[test]
fn test_git_hash() {
    println!("FULL_HASH: {:?}", hash_str());
    println!("SHORT_HASH: {:?}", short_hash_str());
}
