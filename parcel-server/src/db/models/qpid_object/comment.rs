use std::num::TryFromIntError;

use diesel::{Identifiable, Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::{qpid_object_comment_phrases, qpid_object_comments};

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = qpid_object_comments)]
pub struct Comment {
    pub id: i64,
    pub object_id: String,
    pub writer: String,
    pub likes: i64,
    pub parent_index: i16,
    pub is_deleted: bool,
    pub reference_object: String,
}

impl Comment {
    /// Try to convert self into the equivalent api type. Phrases will be set to empty vec.
    ///
    /// Since `likes` is i32 in the api but stored as i64 in the database,
    /// this will return an error if db value is out of range of i32.
    ///
    /// Similarly, `parent_index` is stored as i16 but api type is i8.
    pub fn try_into_api_type(self) -> Result<api_types::object::Comment, TryFromIntError> {
        Ok(api_types::object::Comment {
            phrases: Vec::new(),
            writer: self.writer,
            likes: self.likes.try_into()?,
            parent_index: self.parent_index.try_into()?,
            is_deleted: self.is_deleted,
            reference_object: self.reference_object,
        })
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_comments)]
pub struct NewComment<'a> {
    pub object_id: &'a str,
    pub writer: &'a str,
    pub likes: i64,
    pub parent_index: i16,
    pub is_deleted: bool,
    pub reference_object: &'a str,
}

#[derive(Debug, Queryable, Identifiable)]
#[diesel(table_name = qpid_object_comment_phrases)]
pub struct Phrase {
    pub id: i64,
    pub comment_id: i64,
    pub phrase: i32,
    pub sort_order: i16,
}

impl Phrase {
    pub fn into_api_type(self) -> i32 {
        self.phrase
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_comment_phrases)]
pub struct NewPhrase {
    pub comment_id: i64,
    pub phrase: i32,
    pub sort_order: i16,
}
