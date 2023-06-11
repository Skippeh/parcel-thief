use std::{
    fmt::Display,
    path::{Path, PathBuf},
    time::Duration,
};

use bincode::Options;
use parcel_common::api_types::auth::Provider;

use crate::session::Session;

pub struct SessionStore {
    sessions: moka::future::Cache<String, Session>,
    provider_lookup: moka::future::Cache<String, String>,
    file_path: PathBuf,
}

#[derive(Debug, thiserror::Error)]
pub struct InvalidTokenError;

impl Display for InvalidTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The token must be non zero length")
    }
}

impl SessionStore {
    /// Loads or creates a new session store.
    ///
    /// If loading fails due to an IO or serialization error a new store is created.
    pub async fn load_or_create(file_path: &Path) -> Self {
        // todo: try to load sessions from file
        let sessions = load_sessions(file_path).await.unwrap_or_default();

        let result = Self {
            file_path: file_path.to_owned(),
            sessions: moka::future::CacheBuilder::new(u64::MAX)
                .time_to_live(Duration::from_secs(60 * 60 * 24))
                .name("Sessions")
                .build(),
            provider_lookup: moka::future::CacheBuilder::new(u64::MAX)
                .time_to_live(Duration::from_secs(60 * 60 * 24))
                .name("ProviderLookup")
                .build(),
        };

        let mut futures = Vec::new();

        for session in sessions {
            futures.push(result.save_session(session));
        }

        futures::future::join_all(futures).await;

        result
    }

    pub async fn save_to_file(&self) -> Result<(), anyhow::Error> {
        let sessions = self
            .sessions
            .iter()
            .map(|(_, session)| session)
            .collect::<Vec<_>>();

        save_sessions(&self.file_path, &sessions).await?;

        Ok(())
    }

    pub async fn save_session(&self, session: Session) {
        self.set_reverse_lookup_token(
            session.provider,
            &session.provider_id,
            session.token.clone(),
        )
        .await;
        self.sessions
            .insert(self.get_session_key(&session.token), session)
            .await;

        let _ = self.save_to_file().await;
    }

    pub async fn load_session(&self, token: &str) -> Option<Session> {
        self.sessions.get(token)
    }

    pub async fn delete_session(&self, token: &str) {
        let session = self.sessions.remove(&self.get_session_key(token)).await;

        if let Some(session) = session {
            let reverse_lookup_key =
                self.get_session_reverse_lookup_key(session.provider, &session.provider_id);

            self.provider_lookup.remove(&reverse_lookup_key).await;
        }

        let _ = self.save_to_file().await;
    }

    #[inline]
    pub fn find_active_session_token(
        &self,
        provider: Provider,
        provider_id: &str,
    ) -> Option<String> {
        self.get_reverse_lookup_token(provider, provider_id)
    }

    async fn set_reverse_lookup_token(&self, provider: Provider, provider_id: &str, token: String) {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        self.provider_lookup.insert(key, token).await;
    }

    fn get_reverse_lookup_token(&self, provider: Provider, provider_id: &str) -> Option<String> {
        let key = self.get_session_reverse_lookup_key(provider, provider_id);
        self.provider_lookup.get(&key)
    }

    #[inline]
    fn get_session_key(&self, token: &str) -> String {
        token.to_owned()
    }

    #[inline]
    fn get_session_reverse_lookup_key(&self, provider: Provider, provider_id: &str) -> String {
        format!("{:?}_{}", provider, provider_id)
    }
}

fn get_bincode_options() -> impl bincode::Options {
    bincode::DefaultOptions::new()
        .with_varint_encoding()
        .with_little_endian()
}

async fn load_sessions(file_path: &Path) -> Result<Vec<Session>, anyhow::Error> {
    let bincode = get_bincode_options();
    let bytes = tokio::fs::read(file_path).await?;
    let sessions = bincode.deserialize(&bytes)?;

    Ok(sessions)
}

async fn save_sessions(file_path: &Path, sessions: &[Session]) -> Result<(), anyhow::Error> {
    let bincode = get_bincode_options();
    let bytes = bincode.serialize(sessions)?;
    tokio::fs::write(file_path, bytes).await?;
    Ok(())
}
