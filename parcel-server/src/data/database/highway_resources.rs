use chrono::Utc;
use diesel::prelude::*;
use parcel_common::api_types::requests::devote_highway_resources::PutHistory;

use crate::db::{
    models::highway::{NewDevotedHighwayResources, NewTotalHighwayResources},
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

        conn.transaction(|conn| {
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
                    .execute(conn)?;

                self.add_total_resources(
                    conn,
                    resource.construction_id,
                    resource.resource_id,
                    resource.put_num as i64,
                )?;
            }

            Ok(())
        })
    }

    fn add_total_resources(
        &self,
        conn: &mut PgConnection,
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
            .execute(conn)?;

        Ok(())
    }
}
