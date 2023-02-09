use diesel::prelude::*;

#[derive(Queryable)]
pub struct Account {
    pub id: String,
    pub steam_id: i64,
}
