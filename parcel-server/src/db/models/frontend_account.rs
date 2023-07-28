use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types::auth::Provider;

use crate::db::schema::{
    frontend_account_credentials, frontend_account_provider_connections, frontend_account_sessions,
    frontend_accounts,
};

#[derive(Debug, Queryable)]
pub struct FrontendAccount {
    pub id: i64,
    pub game_account_id: Option<String>,
    pub created_at: NaiveDateTime,
    /// Use `FlagSet<FrontendPermissions>` to read/write flags
    pub permissions: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = frontend_accounts)]
pub struct NewFrontendAccount<'a> {
    pub game_account_id: Option<&'a str>,
    pub created_at: Option<&'a NaiveDateTime>,
    pub permissions: i64,
}

#[derive(Debug, AsChangeset, Default)]
#[diesel(table_name = frontend_accounts)]
pub struct ChangeFrontendAccount<'a> {
    pub game_account_id: Option<Option<&'a str>>,
    pub created_at: Option<&'a NaiveDateTime>,
    pub permissions: Option<i64>,
}

#[derive(Debug, Queryable)]
pub struct AccountCredentials {
    pub account_id: i64,
    pub username: String,
    pub password: String,
    pub salt: Vec<u8>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = frontend_account_credentials)]
pub struct NewAccountCredentials<'a> {
    pub account_id: i64,
    pub username: &'a str,
    pub password: &'a str,
    pub salt: Vec<u8>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Queryable)]
pub struct AccountProviderConnection {
    pub account_id: i64,
    pub provider: Provider,
    pub provider_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = frontend_account_provider_connections)]
pub struct NewAccountProviderConnection<'a> {
    pub account_id: i64,
    pub provider: Provider,
    pub provider_id: &'a str,
    pub created_at: Option<&'a NaiveDateTime>,
}

#[derive(Debug, Queryable)]
pub struct AccountSession {
    pub id: i64,
    pub account_id: i64,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    token: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = frontend_account_sessions)]
pub struct NewAccountSession<'a> {
    pub account_id: i64,
    pub created_at: Option<&'a NaiveDateTime>,
    pub expires_at: &'a NaiveDateTime,
    pub token: &'a str,
}
