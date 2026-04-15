use hex;
use sha2::{Digest, Sha256};
use uuid::Uuid;
use anyhow::Result;

pub fn generate_token() -> Result<String> {
    Ok(Uuid::new_v4().to_string())
}

pub fn hash_token(token: &str) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    Ok(hex::encode(hasher.finalize()))
}
