use diesel::prelude::*;

use crate::db::schema::player_profiles::dsl;
use crate::db::{models::player_profile::PlayerProfile, QueryError};

use super::DatabaseConnection;

pub struct PlayerProfiles<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> PlayerProfiles<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn get_by_account_id(
        &self,
        account_id: &str,
    ) -> Result<Option<PlayerProfile>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let result = dsl::player_profiles
            .find(account_id)
            .first(conn)
            .optional()?;

        Ok(result)
    }

    pub async fn get_by_account_ids(
        &self,
        account_ids: &[impl AsRef<str>],
    ) -> Result<Vec<PlayerProfile>, QueryError> {
        let account_ids: Vec<&str> = account_ids.iter().map(|id| id.as_ref()).collect();
        let conn = &mut *self.connection.get_pg_connection().await;
        let results = dsl::player_profiles
            .filter(dsl::account_id.eq_any(&account_ids))
            .get_results(conn)?;

        Ok(results)
    }

    pub async fn add_or_update_profile(&self, profile: &PlayerProfile) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        diesel::insert_into(dsl::player_profiles)
            .values(profile)
            .on_conflict(dsl::account_id)
            .do_update()
            .set(profile)
            .execute(conn)?;

        Ok(())
    }
}
