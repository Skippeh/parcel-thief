use std::{fmt::Display, sync::Arc};

use diesel::prelude::*;

use crate::db::{
    models::account::{Account, NewAccount},
    schema::accounts,
    QueryError,
};

use super::database::Database;

pub struct Accounts {
    database: Arc<Database>,
}

impl Accounts {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    /// Creates a new account with a randomized id and saves it to the database.
    pub async fn create(&self, steam_id: i64) -> Result<Account, QueryError> {
        if steam_id <= 0 {
            panic!("Invalid steam id, needs to be greater than zero");
        }

        let conn = &mut self.database.connect()?;

        let account = diesel::insert_into(accounts::table)
            .values(&NewAccount {
                id: &generate_account_id(),
                steam_id: &steam_id,
            })
            .get_result(conn)?;

        Ok(account)
    }

    pub async fn get_by_steam_id(&self, steam_id: i64) -> Result<Option<Account>, QueryError> {
        let conn = &mut self.database.connect()?;
        let account = accounts::table
            .filter(accounts::steam_id.eq(&steam_id))
            .first::<Account>(conn)
            .optional()?;

        Ok(account)
    }
}

/// Generates a 32 character long account id
fn generate_account_id() -> String {
    "test".into() // todo
}
