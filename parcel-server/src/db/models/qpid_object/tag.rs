use diesel::{Identifiable, Insertable, Queryable};

use crate::db::schema::qpid_object_tags;

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = qpid_object_tags)]
pub struct Tag {
    pub id: i64,
    pub object_id: String,
    pub tag: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_tags)]
pub struct NewTag<'a> {
    pub object_id: &'a str,
    pub tag: &'a str,
}
