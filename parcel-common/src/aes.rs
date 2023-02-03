use std::sync::{Arc, RwLock};

use aes_gcm::{aead::Aead, AeadInPlace, Aes256Gcm, KeyInit, Nonce};
use anyhow::{Context, Result};
use base64::Engine;
use lazy_static::lazy_static;
use rand::{Rng, RngCore, SeedableRng};
use rand_chacha::ChaCha12Rng;

use crate::api_types::EncryptedData;

const AES_SECRET: &[u8] = &[
    0x4C, 0x48, 0x77, 0x55, 0x47, 0x6E, 0x6B, 0x74, 0x43, 0x6C, 0x4E, 0x76, 0x39, 0x55, 0x6F, 0x63,
    0x31, 0x47, 0x71, 0x7A, 0x63, 0x63, 0x62, 0x68, 0x72, 0x64, 0x61, 0x4A, 0x33, 0x41, 0x06, 0x0A,
];

struct EncryptedMessageParts<'a> {
    nonce: &'a [u8],
    encrypted_data_with_tag: &'a [u8],
}

pub fn decrypt_json_response(body: &str) -> Result<Option<String>> {
    let response = serde_json::from_str::<EncryptedData>(body)
        .context("could not deserialize response json body")?;
    match response.data {
        Some(data) => decrypt_json_data(&data).map(Some),
        None => Ok(None),
    }
}

pub fn decrypt_json_data(encrypted_data: &str) -> Result<String> {
    let mut buffer = vec![0; base64::decoded_len_estimate(encrypted_data.len())];
    let parts =
        decode_data(encrypted_data, &mut buffer).context("could not decode message from base64")?;

    decrypt_data(parts.nonce, parts.encrypted_data_with_tag)
}

fn decode_data<'a>(base64_str: &str, buffer: &'a mut [u8]) -> Result<EncryptedMessageParts<'a>> {
    let b64 = base64::prelude::BASE64_STANDARD;
    let buf_len = b64
        .decode_slice(base64_str, buffer)
        .context("could not decode encrypted base64 string")?;

    // a complete message is guaranteed to have atleast 29 bytes
    // this is because the nonce and auth tag takes up 28 bytes
    if buf_len < 29 {
        anyhow::bail!("incomplete message received");
    }

    let nonce = &buffer[..12];
    let encrypted_data_with_tag = &buffer[12..buf_len];

    Ok(EncryptedMessageParts {
        nonce,
        encrypted_data_with_tag,
    })
}

fn decrypt_data(nonce: &[u8], encrypted_data_with_tag: &[u8]) -> Result<String> {
    if nonce.len() != 12 {
        anyhow::bail!("invalid nonce length");
    }

    let gcm = Aes256Gcm::new_from_slice(AES_SECRET).unwrap();
    let decrypted_bytes = gcm.decrypt(nonce.into(), encrypted_data_with_tag);

    match decrypted_bytes {
        Ok(decrypted_bytes) => Ok(String::from_utf8(decrypted_bytes)?),
        Err(_) => anyhow::bail!("decryption unsuccessful (no reason available)"),
    }
}

lazy_static! {
    static ref NONCE_RNG: Arc<RwLock<ChaCha12Rng>> =
        Arc::new(RwLock::new(ChaCha12Rng::from_entropy()));
}

/// Encrypts json data and returns it as a base64 string
pub fn encrypt_json_data(utf8_bytes: &[u8]) -> String {
    let mut message = vec![0u8; 28 + utf8_bytes.len()];

    // fill first 12 bytes with random values
    NONCE_RNG.write().unwrap().fill_bytes(&mut message[..12]);

    // encrypt data using the generated nonce
    let gcm = Aes256Gcm::new_from_slice(AES_SECRET).unwrap();
    let encrypted_bytes = gcm.encrypt(message[..12].into(), utf8_bytes).unwrap();
    message[12..].copy_from_slice(&encrypted_bytes);

    // return message as a base64 encoded string
    let mut base64 =
        String::with_capacity(base64::encoded_len(message.len(), true).unwrap_or_default());
    base64::prelude::BASE64_STANDARD.encode_string(message, &mut base64);

    base64
}
