use diesel::prelude::*;

use crate::db::schema::accounts;

#[derive(Queryable)]
pub struct Account {
    pub id: String,
    pub steam_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount<'a> {
    pub id: &'a str,
    pub steam_id: &'a i64,
}
