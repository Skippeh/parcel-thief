use chrono::NaiveDateTime;
use diesel::prelude::*;
use parcel_common::api_types::auth::Provider;

use crate::db::schema::accounts;

#[derive(Queryable)]
pub struct Account {
    pub id: String,
    pub display_name: String,
    pub provider: Provider,
    pub provider_id: String,
    pub last_login_date: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub id: &'a str,
    pub display_name: &'a str,
    pub provider: &'a Provider,
    pub provider_id: &'a str,
    pub last_login_date: &'a NaiveDateTime,
}
