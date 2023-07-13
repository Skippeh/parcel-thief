use std::{ops::Deref, path::Path};

use hmac::{Hmac, Mac};
use jwt::{SigningAlgorithm, VerifyingAlgorithm};
use sha2::Sha256;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug)]
pub struct JwtSecret(Hmac<Sha256>);

impl JwtSecret {
    pub async fn load_or_generate_secret() -> Result<Self, anyhow::Error> {
        let secret_path = Path::new("data").join("jwt_secret");

        if secret_path.try_exists()? {
            log::info!("Loading jwt secret from file");

            let mut secret_bytes = Vec::with_capacity(32);
            File::open(&secret_path)
                .await?
                .read_to_end(&mut secret_bytes)
                .await?;

            if secret_bytes.len() != 32 {
                anyhow::bail!("Invalid jwt secret length, expected 32 bytes");
            }

            Ok(Self(Hmac::new_from_slice(&secret_bytes)?))
        } else {
            log::info!("Generating new jwt secret");

            let secret_bytes = parcel_common::rand::generate_u8(32);
            let hmac =
                Hmac::new_from_slice(&secret_bytes).expect("Key length should always be 32 bytes");

            let mut file = File::create(secret_path).await?;
            file.write_all(&secret_bytes).await?;

            Ok(Self(hmac))
        }
    }
}

impl Deref for JwtSecret {
    type Target = Hmac<Sha256>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SigningAlgorithm for JwtSecret {
    fn algorithm_type(&self) -> jwt::AlgorithmType {
        SigningAlgorithm::algorithm_type(&self.0)
    }

    fn sign(&self, header: &str, claims: &str) -> Result<String, jwt::Error> {
        self.0.sign(header, claims)
    }
}

impl VerifyingAlgorithm for JwtSecret {
    fn algorithm_type(&self) -> jwt::AlgorithmType {
        VerifyingAlgorithm::algorithm_type(&self.0)
    }

    fn verify_bytes(
        &self,
        header: &str,
        claims: &str,
        signature: &[u8],
    ) -> Result<bool, jwt::Error> {
        self.0.verify_bytes(header, claims, signature)
    }
}
