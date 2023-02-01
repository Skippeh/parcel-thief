use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use anyhow::{Context, Result};
use base64::Engine;
use parcel_common::api_types::EncryptedData;

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
        Some(data) => {
            let mut buffer = vec![0; base64::decoded_len_estimate(data.len())];
            let parts =
                decode_data(&data, &mut buffer).context("could not decode message from base64")?;

            decrypt_data(parts.nonce, parts.encrypted_data_with_tag).map(Some)
        }
        None => Ok(None),
    }
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
