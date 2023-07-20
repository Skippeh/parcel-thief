use std::path::Path;

use pbkdf2::pbkdf2_hmac_array;
use sha2::Sha256;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

pub struct HashSecret(Vec<u8>);

impl HashSecret {
    pub async fn load_or_generate_secret() -> Result<Self, anyhow::Error> {
        let secret_path = Path::new("data").join("hash_secret");

        if secret_path.try_exists()? {
            log::info!("Loading hash secret from file");

            let mut secret_bytes = Vec::with_capacity(64);
            File::open(&secret_path)
                .await?
                .read_to_end(&mut secret_bytes)
                .await?;

            if secret_bytes.len() != 64 {
                anyhow::bail!("Invalid hash secret length, expected 64 bytes");
            }

            Ok(Self(secret_bytes))
        } else {
            log::warn!("Generating new hash secret");

            let secret_bytes = parcel_common::rand::generate_u8(64);

            let mut file = File::create(secret_path).await?;
            file.write_all(&secret_bytes).await?;

            Ok(Self(secret_bytes))
        }
    }

    pub fn hash_string(&self, str: &str, salt: &[u8]) -> [u8; 64] {
        self.hash_bytes(str.as_bytes(), salt)
    }

    pub fn hash_bytes(&self, bytes: &[u8], salt: &[u8]) -> [u8; 64] {
        let mut salt_with_secret = Vec::with_capacity(salt.len() + self.0.len());
        salt_with_secret.extend_from_slice(salt);
        salt_with_secret.extend_from_slice(&self.0);

        pbkdf2_hmac_array::<Sha256, 64>(bytes, &salt_with_secret, 20_000)
    }
}
