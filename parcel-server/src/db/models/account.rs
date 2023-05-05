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
    pub encountered_id: String,
    pub encountered_at: NaiveDateTime,
}

impl AccountHistory {
    pub fn into_api_type(
        self,
    ) -> parcel_common::api_types::requests::get_relationships::RelationshipHistory {
        parcel_common::api_types::requests::get_relationships::RelationshipHistory {
            last_interaction_time: self.encountered_at.timestamp_millis(),
            account_id: self.encountered_id,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = account_histories)]
pub struct NewAccountHistory<'a> {
    pub account_id: &'a str,
    pub encountered_id: &'a str,
    pub encountered_at: &'a NaiveDateTime,
}

#[derive(Debug, Queryable)]
pub struct AccountStrandContract {
    pub id: i64,
    pub owner_account_id: String,
    pub contract_account_id: String,
    pub created_at: NaiveDateTime,
}

impl AccountStrandContract {
    pub fn into_api_type(
        self,
    ) -> parcel_common::api_types::requests::get_relationships::StrandContract {
        parcel_common::api_types::requests::get_relationships::StrandContract {
            added_time: self.created_at.timestamp_millis(),
            account_id: self.contract_account_id,
        }
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = account_strand_contracts)]
pub struct NewAccountStrandContract<'a> {
    pub owner_account_id: &'a str,
    pub contract_account_id: &'a str,
    pub created_at: &'a NaiveDateTime,
}
