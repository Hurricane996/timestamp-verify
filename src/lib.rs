use std::env;

use base64::{prelude::BASE64_URL_SAFE, DecodeError, Engine};
use hmac::{digest::MacError, Hmac, Mac};
use sha2::Sha256;
use thiserror::Error;

const SIGNATURE_LENGTH_BYTES: usize = 12;
const SIGNATURE_LENGTH_BASE64: usize = 4 * div_rounding_up(SIGNATURE_LENGTH_BYTES, 3);

const fn div_rounding_up(n: usize, d: usize) -> usize {
    if n % d > 0 {
        n / d + 1
    } else {
        n / d
    }
}

#[cfg(test)]
mod tests;

pub(crate) fn make_mac() -> Hmac<Sha256> {
    let key = env!("SECRET_KEY");

    Hmac::<Sha256>::new_from_slice(&key.as_bytes()).unwrap()
}

#[must_use]
pub fn sign(timestamp: i64) -> String {
    let mut mac = make_mac();

    mac.update(&timestamp.to_ne_bytes());

    let result = mac.finalize();

    let bytes = result.into_bytes();

    // 12 bytes for 16 base64 characters
    BASE64_URL_SAFE.encode(&bytes[0..SIGNATURE_LENGTH_BYTES])
}

#[derive(Error, Debug)]
pub enum VerificationError {
    #[error("Failed to decode token; probably mistyped")]
    Base64DecodeError(#[from] DecodeError),
    #[error("Token is not the correct length; probably mistyped")]
    TokenLengthError,
    #[error("Token does not match expected value")]
    HashVerificationError(#[from] MacError),
}

pub fn verify(timestamp: i64, signature: &str) -> Result<i64, VerificationError> {
    if signature.len() != SIGNATURE_LENGTH_BASE64 {
        return Err(VerificationError::TokenLengthError);
    }

    let sig_bytes = BASE64_URL_SAFE
        .decode(signature.trim())
        .map_err(VerificationError::Base64DecodeError)?;

    let mut mac = make_mac();

    mac.update(&timestamp.to_ne_bytes());

    mac.verify_truncated_left(&sig_bytes)
        .map_err(VerificationError::HashVerificationError)?;

    Ok(timestamp)
}
