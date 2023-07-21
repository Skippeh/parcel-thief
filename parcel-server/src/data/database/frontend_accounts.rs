use std::collections::HashMap;

use diesel::{dsl::exists, prelude::*, select};
use flagset::FlagSet;
use parcel_common::api_types::{auth::Provider, frontend::auth::FrontendPermissions};

use crate::{
    data::hash_secret::HashSecret,
    db::{
        models::{
            account::Account as GameAccount,
            frontend_account::{
                AccountCredentials, AccountProviderConnection, FrontendAccount,
                NewAccountCredentials, NewAccountProviderConnection, NewFrontendAccount,
            },
        },
        QueryError,
    },
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

    pub async fn get_by_credentials(
        &self,
        username: &str,
        password: &str,
        hash_secret: &HashSecret,
    ) -> Result<Option<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let salt: Option<Vec<u8>> = {
            dsl::frontend_account_credentials
                .filter(dsl::username.eq(username))
                .select(dsl::salt)
                .first(conn)
                .optional()?
        };

        match salt {
            None => Ok(None),
            Some(salt) => {
                let password_hash = hex::encode(hash_secret.hash_string(password, &salt));

                let account_id: Option<i64> = dsl::frontend_account_credentials
                    .filter(dsl::username.eq(username))
                    .filter(dsl::password.eq(password_hash))
                    .select(dsl::account_id)
                    .first(conn)
                    .optional()?;

                match account_id {
                    None => Ok(None),
                    Some(account_id) => {
                        use crate::db::schema::frontend_accounts::dsl;

                        let account: FrontendAccount = dsl::frontend_accounts
                            .filter(dsl::id.eq(account_id))
                            .first(conn)?; // no need for optional at this point

                        Ok(Some(account))
                    }
                }
            }
        }
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

    pub async fn get_by_id(&self, id: i64) -> Result<Option<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let account = dsl::frontend_accounts
            .filter(dsl::id.eq(id))
            .first(conn)
            .optional()?;

        Ok(account)
    }

    pub async fn get_provider_connection(
        &self,
        account_id: i64,
    ) -> Result<Option<AccountProviderConnection>, QueryError> {
        use crate::db::schema::frontend_account_provider_connections::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let connection = dsl::frontend_account_provider_connections
            .filter(dsl::account_id.eq(account_id))
            .first(conn)
            .optional()?;

        Ok(connection)
    }

    pub async fn get_credentials(
        &self,
        account_id: i64,
    ) -> Result<Option<AccountCredentials>, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let credentials = dsl::frontend_account_credentials
            .filter(dsl::account_id.eq(account_id))
            .first(conn)
            .optional()?;

        Ok(credentials)
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

    pub async fn set_permissions(
        &self,
        account_id: i64,
        permissions: impl Into<FlagSet<FrontendPermissions>>,
    ) -> Result<FlagSet<FrontendPermissions>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let bits = permissions.into().bits();

        let result: i64 = diesel::update(dsl::frontend_accounts)
            .filter(dsl::id.eq(account_id))
            .set(dsl::permissions.eq(bits))
            .returning(dsl::permissions)
            .get_result(conn)?;

        Ok(FlagSet::new_truncated(result))
    }

    pub async fn username_exists(&self, username: &str) -> Result<bool, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        Ok(select(exists(
            dsl::frontend_account_credentials.filter(dsl::username.eq(username)),
        ))
        .get_result(conn)?)
    }

    /// Adds credentials to the frontend account with the given id.
    ///
    /// The password is hashed with the secret key and also with a randomly generated salt which is stored with the credentials.
    pub async fn create_credentials(
        &self,
        account_id: i64,
        username: &str,
        password: &str,
        hash_secret: &HashSecret,
    ) -> Result<AccountCredentials, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let salt = parcel_common::rand::generate_u8(64);
        let password_hash = hex::encode(hash_secret.hash_string(password, &salt));

        let credentials = diesel::insert_into(dsl::frontend_account_credentials)
            .values(&NewAccountCredentials {
                account_id,
                username,
                password: &password_hash,
                salt,
                updated_at: None,
            })
            .get_result(conn)?;

        Ok(credentials)
    }

    pub async fn set_credentials_password(
        &self,
        account_id: i64,
        password: &str,
        hash_secret: &HashSecret,
    ) -> Result<(), QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let salt = parcel_common::rand::generate_u8(64);
        let password_hash = hex::encode(hash_secret.hash_string(password, &salt));

        diesel::update(dsl::frontend_account_credentials)
            .filter(dsl::account_id.eq(account_id))
            .set((
                dsl::password.eq(&password_hash),
                dsl::salt.eq(&salt),
                dsl::updated_at.eq(diesel::dsl::now),
            ))
            .execute(conn)?;

        Ok(())
    }

    /// Query the names for the specified frontend accounts.
    ///
    /// * Accounts with a game account id will use their provider/in-game names
    /// * Accounts without a game account id will use their login names (if any)
    /// * Accounts without a game account id or login names will not be added to the returned hash map
    pub async fn get_display_names(
        &self,
        accounts: &[&FrontendAccount], // todo: remove the need to pass a whole FrontendAccount struct and only require account_id and game_account_id
    ) -> Result<HashMap<i64, String>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        // Get the account ids where the game account id is Some
        let game_account_ids = accounts
            .iter()
            .filter(|account| account.game_account_id.is_some())
            .map(|account| {
                (
                    account
                        .game_account_id
                        .as_ref()
                        .expect("Game account id should always be Some"),
                    account.id,
                )
            })
            .collect::<HashMap<_, _>>();

        let credential_account_ids = accounts
            .iter()
            .filter(|account| account.game_account_id.is_none())
            .map(|account| account.id)
            .collect::<Vec<_>>();

        // Query names from game accounts
        let game_account_names: HashMap<i64, String> = {
            use crate::db::schema::accounts::dsl;

            if game_account_ids.is_empty() {
                HashMap::default()
            } else {
                dsl::accounts
                    .filter(dsl::id.eq_any(game_account_ids.keys()))
                    .select((dsl::id, dsl::display_name))
                    .get_results::<(String, String)>(conn)?
                    .into_iter()
                    .map(|(id, display_name)| {
                        (
                            *game_account_ids
                                .get(&id)
                                .expect("Game account ids should always contain the account id"),
                            display_name,
                        )
                    })
                    .collect()
            }
        };

        // Query usernames from accounts without a game account id
        let usernames: HashMap<i64, String> = {
            use crate::db::schema::frontend_account_credentials::dsl;

            if credential_account_ids.is_empty() {
                HashMap::default()
            } else {
                dsl::frontend_account_credentials
                    .select(dsl::account_id.eq_any(&credential_account_ids))
                    .select((dsl::account_id, dsl::username))
                    .get_results(conn)?
                    .into_iter()
                    .collect()
            }
        };

        let mut result = HashMap::new();
        result.extend(game_account_names.into_iter());
        result.extend(usernames.into_iter());

        Ok(result)
    }
}
