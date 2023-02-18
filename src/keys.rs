use anyhow::{Ok, Result};
use jsonwebtoken::{DecodingKey, EncodingKey};
pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new() -> Result<Self> {
        let binding = std::env::var("JWT_SECRET")?;
        let secret = binding.as_bytes();

        Ok(Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        })
    }
}
pub fn encoding() -> EncodingKey {
    Keys::new().unwrap().encoding
}

pub fn decoding() -> DecodingKey {
    Keys::new().unwrap().decoding
}
