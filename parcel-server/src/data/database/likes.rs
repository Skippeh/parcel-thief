use std::fmt::Display;

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

use crate::db::{
    models::like::{Like, NewLike},
    schema::likes::dsl,
    QueryError,
};

use super::DatabaseConnection;

pub struct Likes<'db> {
    connection: &'db DatabaseConnection<'db>,
}

pub enum LikeTarget<'a> {
    Dummy,
    Shared,
    Highway(u32),
    Object(&'a str),
}

#[derive(Debug, thiserror::Error)]
pub struct UnknownObjectType;

impl Display for UnknownObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The format of the string did not match any known object types"
        )
    }
}

impl<'a> TryFrom<&'a str> for LikeTarget<'a> {
    type Error = UnknownObjectType;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some(highway_id_str) = value.strip_prefix('h') {
            let highway_segment_id = highway_id_str
                .parse::<u32>()
                .map_err(|_| UnknownObjectType)?;
            Ok(Self::Highway(highway_segment_id))
        } else if value.eq_ignore_ascii_case("idummy") {
            Ok(Self::Dummy)
        } else if value.eq_ignore_ascii_case("ishared") {
            Ok(Self::Shared)
        } else {
            Ok(Self::Object(value))
        }
    }
}

impl<'db> Likes<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn give_likes(
        &self,
        num_likes_auto: i32,
        num_likes_manual: i32,
        like_type: &str,
        from_id: &str,
        to_id: &str,
        target_online_id: LikeTarget<'_>,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            if let LikeTarget::Object(object_id) = &target_online_id {
                use crate::db::schema::qpid_objects::dsl as object_dsl;

                let mut current_likes = object_dsl::qpid_objects
                    .filter(object_dsl::id.eq(object_id))
                    .select(object_dsl::likes)
                    .first::<i64>(conn)?;

                let total_likes = num_likes_auto as i64 + num_likes_manual as i64;
                current_likes += total_likes;

                diesel::update(object_dsl::qpid_objects)
                    .filter(object_dsl::id.eq(object_id))
                    .set(object_dsl::likes.eq(current_likes))
                    .execute(conn)?;

                log::debug!(
                    "Incremented likes on {}: {} + {} = {}",
                    object_id,
                    current_likes - total_likes,
                    total_likes,
                    current_likes
                )
            }

            let target_online_id = match target_online_id {
                LikeTarget::Dummy => "idummy".into(),
                LikeTarget::Shared => "ishared".into(),
                LikeTarget::Highway(id) => format!("h{id}"),
                LikeTarget::Object(id) => id.into(),
            };

            diesel::insert_into(dsl::likes)
                .values(NewLike {
                    time: &Utc::now().naive_utc(),
                    from_id,
                    to_id,
                    online_id: &target_online_id,
                    likes_manual: num_likes_manual,
                    likes_auto: num_likes_auto,
                    ty: like_type,
                    acknowledged: false,
                })
                .execute(conn)?;

            Ok(())
        })
    }

    pub async fn set_acknowledged(
        &self,
        like_ids: &[i64],
        acknowledged: bool,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        diesel::update(dsl::likes)
            .filter(dsl::id.eq_any(like_ids))
            .set(dsl::acknowledged.eq(acknowledged))
            .execute(conn)?;

        Ok(())
    }

    pub async fn get_likes_since(
        &self,
        account_id: &str,
        since: &NaiveDateTime,
    ) -> Result<Vec<Like>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        Ok(dsl::likes
            .filter(dsl::to_id.eq(account_id))
            .filter(dsl::time.gt(since))
            .get_results(conn)?)
    }

    pub async fn get_unacknowleged_likes(&self, account_id: &str) -> Result<Vec<Like>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        Ok(dsl::likes
            .filter(dsl::to_id.eq(account_id))
            .filter(dsl::acknowledged.eq(false))
            .get_results(conn)?)
    }
}
