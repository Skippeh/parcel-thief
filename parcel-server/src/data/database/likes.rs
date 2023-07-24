use std::{collections::HashMap, fmt::Display};

use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};

use crate::db::{
    models::like::{Like, NewLike, NewTotalHighwayLikes, TotalHighwayLikes},
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
            async move {
                let total_likes = num_likes_auto as i64 + num_likes_manual as i64;

                match &target_online_id {
                    LikeTarget::Object(object_id) => {
                        use crate::db::schema::qpid_objects::dsl as object_dsl;

                        let new_total = diesel::update(object_dsl::qpid_objects)
                            .filter(object_dsl::id.eq(object_id))
                            .set(object_dsl::likes.eq(object_dsl::likes + total_likes))
                            .returning(object_dsl::likes)
                            .get_result::<i64>(conn)
                            .await
                            .optional()?;

                        match new_total {
                            Some(new_total) => {
                                log::debug!(
                                    "Added {} likes to {}, new total = {}",
                                    total_likes,
                                    object_id,
                                    new_total
                                );
                            }
                            None => {
                                log::warn!(
                                    "Could not find object to increment likes on: {}",
                                    object_id
                                )
                            }
                        }
                    }
                    LikeTarget::Highway(_) => {
                        use crate::db::schema::total_highway_likes::dsl;

                        let new_total = diesel::insert_into(dsl::total_highway_likes)
                            .values(&NewTotalHighwayLikes {
                                account_id: to_id,
                                likes: total_likes,
                            })
                            .on_conflict(dsl::account_id)
                            .do_update()
                            .set(dsl::likes.eq(dsl::likes + total_likes))
                            .returning(dsl::likes)
                            .get_result::<i64>(conn)
                            .await?;

                        log::debug!("New total highway likes for {} = {}", to_id, new_total);
                    }
                    _ => (),
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
                    .execute(conn)
                    .await?;

                Ok(())
            }
            .scope_boxed()
        })
        .await
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
            .execute(conn)
            .await?;

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
            .get_results(conn)
            .await?)
    }

    pub async fn get_unacknowleged_likes(&self, account_id: &str) -> Result<Vec<Like>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        Ok(dsl::likes
            .filter(dsl::to_id.eq(account_id))
            .filter(dsl::acknowledged.eq(false))
            .get_results(conn)
            .await?)
    }

    pub async fn get_total_highway_likes<'a>(
        &self,
        account_ids: impl IntoIterator<Item = &str>,
    ) -> Result<HashMap<String, i64>, QueryError> {
        use crate::db::schema::total_highway_likes::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let likes = dsl::total_highway_likes
            .filter(dsl::account_id.eq_any(account_ids))
            .get_results::<TotalHighwayLikes>(conn)
            .await?;

        let mut result = HashMap::new();

        for like in likes {
            result.insert(like.account_id, like.likes);
        }

        Ok(result)
    }
}
