use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::{dsl::exists, prelude::*, select};
use diesel_async::RunQueryDsl;
use flagset::FlagSet;
use parcel_common::api_types::{auth::Provider, frontend::auth::FrontendPermissions};

use crate::{
    data::hash_secret::HashSecret,
    db::{
        models::frontend_account::{
            AccountCredentials, AccountProviderConnection, AccountSession, ChangeFrontendAccount,
            FrontendAccount, NewAccountCredentials, NewAccountProviderConnection,
            NewAccountSession, NewFrontendAccount,
        },
        QueryError,
    },
};

use super::DatabaseConnection;

pub struct FrontendAccounts<'db> {
    connection: &'db DatabaseConnection<'db>,
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
                .await
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
                    .await
                    .optional()?;

                match account_id {
                    None => Ok(None),
                    Some(account_id) => {
                        use crate::db::schema::frontend_accounts::dsl;

                        let account: FrontendAccount = dsl::frontend_accounts
                            .filter(dsl::id.eq(account_id))
                            .first(conn)
                            .await?; // no need for optional at this point

                        Ok(Some(account))
                    }
                }
            }
        }
    }

    pub async fn get_by_provider(
        &self,
        provider: Provider,
        provider_id: &str,
    ) -> Result<Option<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_account_provider_connections::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let connection: Option<AccountProviderConnection> =
            dsl::frontend_account_provider_connections
                .filter(dsl::provider.eq(&provider))
                .filter(dsl::provider_id.eq(provider_id))
                .first(conn)
                .await
                .optional()?;

        match connection {
            None => Ok(None),
            Some(connection) => {
                use crate::db::schema::frontend_accounts::dsl;

                let account = dsl::frontend_accounts
                    .filter(dsl::id.eq(connection.account_id))
                    .first(conn)
                    .await?;

                Ok(Some(account))
            }
        }
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let account = dsl::frontend_accounts
            .filter(dsl::id.eq(id))
            .first(conn)
            .await
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
            .await
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
            .await
            .optional()?;

        Ok(credentials)
    }

    pub async fn get_all(&self) -> Result<Vec<FrontendAccount>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let accounts = dsl::frontend_accounts.get_results(conn).await?;

        Ok(accounts)
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
            .get_result(conn)
            .await?;

        Ok(FlagSet::new_truncated(result))
    }

    pub async fn username_exists(&self, username: &str) -> Result<bool, QueryError> {
        use crate::db::schema::frontend_account_credentials::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        Ok(select(exists(
            dsl::frontend_account_credentials.filter(dsl::username.eq(username)),
        ))
        .get_result(conn)
        .await?)
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
            .get_result(conn)
            .await?;

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
            .execute(conn)
            .await?;

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
                    .get_results::<(String, String)>(conn)
                    .await?
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
                    .filter(dsl::account_id.eq_any(&credential_account_ids))
                    .select((dsl::account_id, dsl::username))
                    .get_results(conn)
                    .await?
                    .into_iter()
                    .collect()
            }
        };

        let mut result = HashMap::new();
        result.extend(game_account_names.into_iter());
        result.extend(usernames.into_iter());

        Ok(result)
    }

    pub async fn get_ids_from_game_ids(
        &self,
        game_ids: &[&str],
    ) -> Result<HashMap<String, i64>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let account_ids = dsl::frontend_accounts
            .filter(dsl::game_account_id.eq_any(game_ids))
            .select((dsl::game_account_id.assume_not_null(), dsl::id))
            .get_results::<(String, i64)>(conn)
            .await?
            .into_iter()
            .collect();

        Ok(account_ids)
    }

    pub async fn add_account(
        &self,
        account: &NewFrontendAccount<'_>,
    ) -> Result<FrontendAccount, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let account = diesel::insert_into(dsl::frontend_accounts)
            .values(account)
            .get_result(conn)
            .await?;

        Ok(account)
    }

    pub async fn update_account(
        &self,
        account_id: i64,
        updates: &ChangeFrontendAccount<'_>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::update(dsl::frontend_accounts)
            .filter(dsl::id.eq(account_id))
            .set(updates)
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn add_provider_connection(
        &self,
        connection: &NewAccountProviderConnection<'_>,
    ) -> Result<AccountProviderConnection, QueryError> {
        use crate::db::schema::frontend_account_provider_connections::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        let connection = diesel::insert_into(dsl::frontend_account_provider_connections)
            .values(connection)
            .get_result(conn)
            .await?;

        Ok(connection)
    }

    /// Creates an account with all permissions if no account with the ManageAccounts permission is found.
    ///
    /// At the moment the account username is always set to "admin" but this may change in the future.
    ///
    /// The returned value is the username and unhashed password of the created account.
    pub async fn create_admin_account_if_not_exists(
        &self,
        hash_secret: &HashSecret,
    ) -> Result<Option<(String, String)>, QueryError> {
        use crate::db::schema::frontend_accounts::dsl;

        let mut conn_guard = self.connection.get_pg_connection().await;

        let permissions = dsl::frontend_accounts
            .select(dsl::permissions)
            .get_results::<i64>(&mut *conn_guard)
            .await?
            .into_iter()
            .map(|perm| FlagSet::<FrontendPermissions>::new_truncated(perm))
            .collect::<Vec<_>>();

        let mut create_account = true;

        for permission in permissions {
            if permission.contains(FrontendPermissions::ManageAccounts) {
                create_account = false;
                break;
            }
        }

        if create_account {
            let username = "admin".to_string();
            let password = generate_password();
            // using i64::MAX will make this account also inherit any new permissions added later in time,
            // as long as the permissions are not edited in the frontend.
            let permissions = i64::MAX;

            let account: FrontendAccount = diesel::insert_into(dsl::frontend_accounts)
                .values(&NewFrontendAccount {
                    game_account_id: None,
                    created_at: None,
                    permissions,
                })
                .get_result(&mut *conn_guard)
                .await?;

            // drop guard to avoid deadlock
            std::mem::drop(conn_guard);

            self.create_credentials(account.id, &username, &password, hash_secret)
                .await?;

            Ok(Some((username, password)))
        } else {
            Ok(None)
        }
    }

    pub async fn add_session(
        &self,
        account_id: i64,
        auth_token: &str,
        expires_at: &NaiveDateTime,
    ) -> Result<(), QueryError> {
        use crate::db::schema::frontend_account_sessions::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::insert_into(dsl::frontend_account_sessions)
            .values(&NewAccountSession {
                account_id,
                created_at: None,
                expires_at,
                token: auth_token,
            })
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn get_sessions_by_account_id(
        &self,
        account_id: i64,
    ) -> Result<Vec<AccountSession>, QueryError> {
        use crate::db::schema::frontend_account_sessions::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        Ok(dsl::frontend_account_sessions
            .filter(dsl::account_id.eq(account_id))
            .get_results(conn)
            .await?)
    }

    pub async fn delete_session_by_token(&self, token: &str) -> Result<(), QueryError> {
        use crate::db::schema::frontend_account_sessions::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::delete(dsl::frontend_account_sessions)
            .filter(dsl::token.eq(token))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_sessions_by_id(&self, ids: &[i64]) -> Result<(), QueryError> {
        use crate::db::schema::frontend_account_sessions::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::delete(dsl::frontend_account_sessions.filter(dsl::id.eq_any(ids)))
            .execute(conn)
            .await?;

        Ok(())
    }

    pub async fn delete_expired_sessions(&self) -> Result<(), QueryError> {
        use crate::db::schema::frontend_account_sessions::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::delete(dsl::frontend_account_sessions)
            .filter(dsl::expires_at.le(diesel::dsl::now))
            .execute(conn)
            .await?;

        Ok(())
    }
}

fn generate_password() -> String {
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    parcel_common::rand::generate_string(12, chars)
}
