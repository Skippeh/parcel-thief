use std::collections::HashMap;

use chrono::{DateTime, Utc};
use diesel::{dsl::not, prelude::*};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use itertools::Itertools;
use parcel_common::api_types::requests::devote_highway_resources::PutHistory;

use crate::db::{
    models::highway::{
        DevotedHighwayResources, NewDevotedHighwayResources, NewTotalHighwayResources,
        TotalHighwayResources,
    },
    QueryError,
};

use super::DatabaseConnection;

pub struct HighwayResources<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> HighwayResources<'db> {
    pub fn new(db: &'db DatabaseConnection<'db>) -> Self {
        Self { connection: db }
    }

    pub async fn devote_resources(
        &self,
        account_id: &str,
        resources: impl IntoIterator<Item = &PutHistory>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::devoted_highway_resources::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;
        let resources = resources.into_iter().collect::<Vec<_>>();

        conn.transaction(|conn| {
            async move {
                let time = Utc::now().naive_utc();

                // Ideally we wouldn't insert one at a time, but realistically only one (up to 3)
                // resource(s) is sent by the client at a time, so i'm not gonna bother for now.
                for resource in resources {
                    diesel::insert_into(dsl::devoted_highway_resources)
                        .values(&NewDevotedHighwayResources {
                            account_id,
                            construction_id: resource.construction_id,
                            time: &time,
                            resource_id: resource.resource_id,
                            num_resources: resource.put_num,
                        })
                        .execute(conn)
                        .await?;

                    self.add_total_resources(
                        conn,
                        resource.construction_id,
                        resource.resource_id,
                        resource.put_num as i64,
                    )
                    .await?;
                }

                Ok(())
            }
            .scope_boxed()
        })
        .await
    }

    pub async fn get_contributors(
        &self,
        constructions_since: impl IntoIterator<Item = (i32, DateTime<Utc>)>,
        resource_ids: &[i16],
        account_id: &str,
        limit: Option<i64>,
    ) -> Result<HashMap<i32, Vec<String>>, QueryError> {
        use crate::db::schema::devoted_highway_resources::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;
        let constructions_since = constructions_since.into_iter().collect::<Vec<_>>();
        let construction_ids = constructions_since
            .iter()
            .map(|id_since| id_since.0)
            .collect::<Vec<_>>();
        let earliest_date = constructions_since
            .iter()
            .min_by_key(|id_since| id_since.1)
            .map(|id_since| id_since.1.naive_utc())
            .unwrap_or_else(|| Utc::now().naive_utc());

        let resources: Vec<DevotedHighwayResources> = dsl::devoted_highway_resources
            .filter(dsl::resource_id.eq_any(resource_ids))
            .filter(dsl::construction_id.eq_any(construction_ids))
            .filter(not(dsl::account_id.eq(account_id)))
            .filter(dsl::time.gt(earliest_date))
            .distinct_on(dsl::account_id)
            .get_results(conn)
            .await?;

        let mut by_construction_id = resources
            .into_iter()
            .into_group_map_by(|r| r.construction_id);

        if let Some(limit) = limit {
            // If there's a way to limit number of results by column in db query then we should do that instead of the following
            for array in by_construction_id.values_mut() {
                if array.len() > limit as usize {
                    array.truncate(limit as usize);
                    array.shrink_to_fit();
                }
            }
        }

        let mut result = HashMap::new();

        for (construction_id, resources) in by_construction_id {
            result.insert(
                construction_id,
                resources.into_iter().map(|res| res.account_id).collect(),
            );
        }

        Ok(result)
    }

    pub async fn get_total_resources(
        &self,
        construction_ids: impl IntoIterator<Item = i32>,
        resource_ids: impl IntoIterator<Item = i16>,
    ) -> Result<Vec<TotalHighwayResources>, QueryError> {
        use crate::db::schema::total_highway_resources::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let resources = dsl::total_highway_resources
            .filter(dsl::construction_id.eq_any(construction_ids.into_iter()))
            .filter(dsl::resource_id.eq_any(resource_ids.into_iter()))
            .get_results(conn)
            .await?;

        Ok(resources)
    }

    async fn add_total_resources(
        &self,
        conn: &mut AsyncPgConnection,
        construction_id: i32,
        resource_id: i16,
        num_resources: i64,
    ) -> Result<(), QueryError> {
        use crate::db::schema::total_highway_resources::dsl;

        diesel::insert_into(dsl::total_highway_resources)
            .values(&NewTotalHighwayResources {
                construction_id,
                resource_id,
                num_resources,
            })
            .on_conflict((dsl::construction_id, dsl::resource_id))
            .do_update()
            .set(dsl::num_resources.eq(dsl::num_resources + num_resources))
            .execute(conn)
            .await?;

        Ok(())
    }
}
