use base64::Engine;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use parcel_common::{api_types::auth::Provider, rand};

use crate::db::{
    models::account::{
        Account, AccountHistory, AccountStrandContract, NewAccount, NewAccountHistory,
        NewAccountStrandContract,
    },
    schema::accounts,
    QueryError,
};

use super::DatabaseConnection;

pub struct Accounts<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> Accounts<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    /// Creates a new account with a randomized id and saves it to the database.
    pub async fn create(
        &self,
        provider: Provider,
        provider_id: &str,
        display_name: &str,
        last_login_date: &NaiveDateTime,
    ) -> Result<Account, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let account = diesel::insert_into(accounts::table)
            .values(&NewAccount {
                id: &generate_account_id(),
                display_name,
                provider: &provider,
                provider_id,
                last_login_date,
            })
            .get_result(conn)?;

        Ok(account)
    }

    pub async fn get_by_provider_id(
        &self,
        provider: Provider,
        provider_id: &str,
    ) -> Result<Option<Account>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let account = accounts::table
            .filter(accounts::provider_id.eq(&provider_id))
            .filter(accounts::provider.eq(&provider))
            .first(conn)
            .optional()?;

        Ok(account)
    }

    pub async fn get_by_provider_ids(
        &self,
        provider: Provider,
        provider_ids: &[impl AsRef<str>],
    ) -> Result<Vec<Account>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let provider_ids: Vec<&str> = provider_ids.iter().map(|id| id.as_ref()).collect();
        let accounts = accounts::table
            .filter(accounts::provider.eq(&provider))
            .filter(accounts::provider_id.eq_any(provider_ids))
            .get_results::<Account>(conn)?;

        Ok(accounts)
    }

    pub async fn get_by_id(&self, account_id: &str) -> Result<Option<Account>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let account = accounts::table
            .filter(accounts::id.eq(account_id))
            .first(conn)
            .optional()?;

        Ok(account)
    }

    pub async fn get_by_ids(
        &self,
        account_ids: &[impl AsRef<str>],
    ) -> Result<Vec<Account>, QueryError> {
        if account_ids.is_empty() {
            return Ok(Vec::new());
        }

        let conn = &mut *self.connection.get_pg_connection().await;
        let account_ids: Vec<&str> = account_ids.iter().map(|id| id.as_ref()).collect();
        let accounts = accounts::table
            .filter(accounts::id.eq_any(account_ids))
            .get_results::<Account>(conn)?;

        Ok(accounts)
    }

    pub async fn update_display_name_and_last_login(
        &self,
        account_id: &str,
        display_name: &str,
        last_login: &NaiveDateTime,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::update(accounts::table.find(account_id))
            .set((
                accounts::display_name.eq(display_name),
                accounts::last_login_date.eq(last_login),
            ))
            .get_result::<Account>(conn)?;

        Ok(())
    }

    pub async fn get_relationship_history(
        &self,
        account_id: &str,
        limit: Option<i64>,
    ) -> Result<Vec<AccountHistory>, QueryError> {
        use crate::db::schema::account_histories::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let account_histories = if let Some(limit) = limit {
            dsl::account_histories
                .filter(dsl::account_id.eq(account_id))
                .order_by(dsl::encountered_at.desc())
                .limit(limit)
                .get_results::<AccountHistory>(conn)?
        } else {
            dsl::account_histories
                .filter(dsl::account_id.eq(account_id))
                .order_by(dsl::encountered_at.desc())
                .get_results::<AccountHistory>(conn)?
        };

        Ok(account_histories)
    }

    pub async fn add_relationship_history(
        &self,
        account_id: &str,
        encountered_id: &str,
        encountered_at: &NaiveDateTime,
    ) -> Result<(), QueryError> {
        use crate::db::schema::account_histories::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::insert_into(dsl::account_histories)
            .values(&NewAccountHistory {
                account_id,
                encountered_id,
                encountered_at,
            })
            .on_conflict((dsl::account_id, dsl::encountered_id))
            .do_update()
            .set(dsl::encountered_at.eq(encountered_at))
            .execute(conn)?;

        Ok(())
    }

    pub async fn add_strand_contracts(
        &self,
        account_id: &str,
        contract_account_ids: impl Iterator<Item = &str>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::account_strand_contracts::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;
        let created_at = chrono::Utc::now().naive_utc();

        diesel::insert_into(dsl::account_strand_contracts)
            .values(
                &contract_account_ids
                    .map(|id| NewAccountStrandContract {
                        owner_account_id: account_id,
                        contract_account_id: id,
                        created_at: &created_at,
                    })
                    .collect::<Vec<NewAccountStrandContract>>(),
            )
            .on_conflict_do_nothing()
            .execute(conn)?;

        Ok(())
    }

    pub async fn remove_strand_contracts(
        &self,
        account_id: &str,
        contract_account_ids: impl Iterator<Item = &str>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::account_strand_contracts::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        diesel::delete(
            dsl::account_strand_contracts
                .filter(dsl::owner_account_id.eq(account_id))
                .filter(dsl::contract_account_id.eq_any(contract_account_ids)),
        )
        .execute(conn)?;

        Ok(())
    }

    pub async fn get_strand_contracts(
        &self,
        account_id: &str,
    ) -> Result<Vec<AccountStrandContract>, QueryError> {
        use crate::db::schema::account_strand_contracts::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        Ok(dsl::account_strand_contracts
            .filter(dsl::owner_account_id.eq(account_id))
            .get_results::<AccountStrandContract>(conn)?)
    }
}

/// Generates a 32 character long account id.
///
/// The first few characters are always zygo_**** (where **** is STATIC_BYTES encoded as base64), followed by random bytes encoded as base64, up to a total of 20 bytes (not including zygo_).
///
/// Note that the real server doesn't follow this logic, it's only done this way because i'm not really sure what the id "is".
///
/// When this value does not "conform" to some format some things don't work in the game, such as displaying player names associated with this id.
/// I haven't been able to figure out what exactly it is but this seems to work from the somewhat limited testing i've done.
fn generate_account_id() -> String {
    const STATIC_BYTES: [u8; 8] = [0xd8, 0x9c, 0x20, 0xf6, 0x97, 0xe0, 0xe6, 0x86];
    let mut id = vec![0; 20];
    id[..8].copy_from_slice(&STATIC_BYTES);
    rand::overwrite_generate_u8(&mut id[8..]);

    let b64 = base64::engine::general_purpose::STANDARD.encode(id);
    format!("zygo_{}", b64)
}
