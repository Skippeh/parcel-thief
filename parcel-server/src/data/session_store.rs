use std::{fmt::Display, path::Path, time::Duration};

use parcel_common::api_types::auth::Provider;

use crate::session::Session;

pub struct SessionStore {
    sessions: moka::future::Cache<String, Session>,
    provider_lookup: moka::future::Cache<String, String>,
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
    pub fn load_or_create(file_path: &Path) -> Self {
        // todo: try to load sessions from file

        Self {
            sessions: moka::future::CacheBuilder::new(u64::MAX)
                .time_to_idle(Duration::from_secs(60 * 60 * 12))
                .name("Sessions")
                .build(),
            provider_lookup: moka::future::CacheBuilder::new(u64::MAX)
                .time_to_idle(Duration::from_secs(60 * 60 * 12))
                .name("ProviderLookup")
                .build(),
        }
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
