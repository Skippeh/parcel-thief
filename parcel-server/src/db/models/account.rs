use chrono::NaiveDateTime;
use diesel::prelude::*;
use parcel_common::api_types::auth::Provider;

use crate::db::schema::{account_histories, account_strand_contracts, accounts};

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

#[derive(Debug, Queryable)]
pub struct AccountHistory {
    pub id: i64,
    pub account_id: String,
    pub encountered_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = account_histories)]
pub struct NewAccountHistory<'a> {
    pub account_id: &'a str,
    pub encountered_at: &'a NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct AccountStrandContract {
    pub id: i64,
    pub owner_account_id: String,
    pub contract_account_id: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = account_strand_contracts)]
pub struct NewAccountStrandContract<'a> {
    pub owner_account_id: &'a str,
    pub contract_account_id: &'a str,
}
