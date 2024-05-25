use hmac::{digest::InvalidLength, Hmac, Mac};
use secrecy::ExposeSecret;
use sha2::Sha256;
use shared::configuration::CONFIG;

use super::random_string::generate;

pub fn generate_fingerprint() -> Result<(String, String), InvalidLength> {
    let fingerprint = generate(50);
    let mut mac =
        Hmac::<Sha256>::new_from_slice(&CONFIG.auth.hmac_secret.expose_secret().as_bytes())?;

    mac.update(&fingerprint.as_bytes());

    let result = mac.finalize();

    let code_bytes = result.into_bytes();

    Ok((fingerprint, format!("{:X}", code_bytes)))
}

pub fn verify_fingerprint_hash(
    fingerprint: &str,
    fingerprint_hash: &str,
) -> Result<bool, InvalidLength> {
    let mut mac =
        Hmac::<Sha256>::new_from_slice(&CONFIG.auth.hmac_secret.expose_secret().as_bytes())?;

    mac.update(&fingerprint.as_bytes());

    let result = mac.finalize();

    let code_bytes = result.into_bytes();

    Ok(fingerprint_hash == format!("{:X}", code_bytes))
}
