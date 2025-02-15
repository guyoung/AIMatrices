//! SHA-256 digest

use std::path::Path;

use sha2::{Digest, Sha256};

/// Return the hex SHA-256 digest of the given bytes.
pub fn hex_digest_from_bytes(bytes: impl AsRef<[u8]>) -> String {
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}

/// Return the hex SHA-256 digest of the given file.
pub fn hex_digest_from_file(path: impl AsRef<Path>) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    std::io::copy(&mut file, &mut hasher)?;
    let digest = hasher.finalize();
    Ok(format!("{digest:x}"))
}
