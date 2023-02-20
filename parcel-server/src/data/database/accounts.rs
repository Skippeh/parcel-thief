use chrono::NaiveDateTime;
use diesel::prelude::*;
use parcel_common::{api_types::auth::Provider, rand};

use crate::db::{
    models::account::{Account, NewAccount},
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
}

/// Generates a 32 character long account id
fn generate_account_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = "zygo_".into();
    rand::append_generate_string(&mut result, 27, CHARS);

    result
}
