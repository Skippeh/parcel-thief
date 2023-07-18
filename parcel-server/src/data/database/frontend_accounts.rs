use std::collections::HashMap;

use diesel::prelude::*;
use parcel_common::api_types::auth::Provider;

use crate::db::{
    models::{
        account::Account as GameAccount,
        frontend_account::{
            AccountCredentials, AccountProviderConnection, FrontendAccount,
            NewAccountProviderConnection, NewFrontendAccount,
        },
    },
    QueryError,
};

use super::DatabaseConnection;

pub struct FrontendAccounts<'db> {
    connection: &'db DatabaseConnection<'db>,
}

#[derive(Debug, thiserror::Error)]
pub enum GetOrCreateError {
    #[error("Account not found")]
    GameAccountNotFound,
    #[error("{0}")]
    QueryError(QueryError),
}

impl From<QueryError> for GetOrCreateError {
    fn from(value: QueryError) -> Self {
        GetOrCreateError::QueryError(value)
    }
}

impl From<diesel::result::Error> for GetOrCreateError {
    fn from(value: diesel::result::Error) -> Self {
        GetOrCreateError::QueryError(value.into())
    }
}

impl<'db> FrontendAccounts<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    /// Gets or creates a frontend account for the given provider account.
    ///
    /// Note that if there's no game account matching the provider account, an error is returned.
    pub async fn get_or_create_from_provider(
        &self,
        provider: Provider,
        provider_id: &str,
    ) -> Result<FrontendAccount, GetOrCreateError> {
        use crate::db::schema::frontend_account_provider_connections::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            let connection: Option<AccountProviderConnection> =
                dsl::frontend_account_provider_connections
                    .filter(dsl::provider.eq(&provider))
                    .filter(dsl::provider_id.eq(provider_id))
                    .first(conn)
                    .optional()?;

            match connection {
                Some(connection) => {
                    use crate::db::schema::frontend_accounts::dsl;

                    let account = dsl::frontend_accounts
                        .filter(dsl::id.eq(connection.account_id))
                        .first(conn)?;

                    Ok(account)
                }
                None => {
                    // Check if there's a game account for this provider id. If there isn't we return an error.
                    use crate::db::schema::accounts::dsl;

                    let account: Option<GameAccount> = dsl::accounts
                        .filter(dsl::provider.eq(&provider))
                        .filter(dsl::provider_id.eq(provider_id))
                        .first(conn)
                        .optional()?;

                    match account {
                        None => Err(GetOrCreateError::GameAccountNotFound),
                        Some(game_account) => {
                            use crate::db::schema::frontend_accounts::dsl;

                            // Create a new frontend account
                            let frontend_account = diesel::insert_into(dsl::frontend_accounts)
                                .values(&NewFrontendAccount {
                                    game_account_id: Some(&game_account.id),
                                    created_at: None,
                                    permissions: 0,
                                })
                                .get_result::<FrontendAccount>(conn)?;

                            // Create the provider connection
                            {
                                use crate::db::schema::frontend_account_provider_connections::dsl;
                                diesel::insert_into(dsl::frontend_account_provider_connections)
                                    .values(&NewAccountProviderConnection {
                                        account_id: frontend_account.id,
                                        provider,
                                        provider_id: &provider_id,
                                        created_at: None,
                                    })
                                    .execute(conn)?;
                            }

                            Ok(frontend_account)
                        }
                    }
                }
            }
        })
    }

    pub async fn get_all(&self) -> Result<Vec<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let accounts = dsl::frontend_accounts.get_results(conn)?;

        Ok(accounts)
    }

    pub async fn get_login_usernames(
        &self,
        account_ids: &[i64],
    ) -> Result<HashMap<i64, String>, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let credentials: Vec<AccountCredentials> = dsl::frontend_account_credentials
            .filter(dsl::account_id.eq_any(account_ids))
            .get_results(conn)?;

        Ok(credentials
            .into_iter()
            .map(|c| (c.account_id, c.username))
            .collect())
    }
}
