use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::Connection;
use parcel_common::api_types::requests::delete_wasted_baggages::DeleteRequest;
use parcel_common::api_types::requests::get_wasted_baggages::WastedItem;

use crate::db::models::wasted_baggage::WastedBaggage;
use crate::db::{models::wasted_baggage::NewWastedBaggage, QueryError};

use super::DatabaseConnection;

pub struct WastedBaggages<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> WastedBaggages<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn add_wasted_items(
        &self,
        qpid_id: i32,
        owner_id: &str,
        items: &[WastedItem],
    ) -> Result<(), QueryError> {
        use crate::db::schema::wasted_baggages::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let created_at = chrono::Utc::now().naive_utc();

        conn.transaction(|conn| {
            diesel::insert_into(dsl::wasted_baggages)
                .values(
                    &items
                        .iter()
                        .map(|item| NewWastedBaggage {
                            id: generate_wasted_baggage_id(),
                            qpid_id,
                            creator_id: owner_id,
                            created_at: &created_at,
                            item_hash: item.item_hash,
                            broken: item.broken,
                            x: item.x,
                            y: item.y,
                            z: item.z,
                        })
                        .collect::<Vec<_>>(),
                )
                .execute(conn)?;

            Ok(())
        })
    }

    pub async fn get_wasted_baggages(
        &self,
        qpid_id: i32,
        earliest_date: Option<&NaiveDateTime>,
    ) -> Result<Vec<WastedBaggage>, QueryError> {
        use crate::db::schema::wasted_baggages::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let baggages = if let Some(earliest_date) = earliest_date {
            dsl::wasted_baggages
                .filter(dsl::qpid_id.eq(qpid_id))
                .filter(dsl::created_at.gt(earliest_date))
                .get_results(conn)?
        } else {
            dsl::wasted_baggages
                .filter(dsl::qpid_id.eq(qpid_id))
                .get_results(conn)?
        };

        Ok(baggages)
    }

    pub async fn delete_by_requests(
        &self,
        delete_requests: impl Iterator<Item = &DeleteRequest>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::wasted_baggages::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            for request in delete_requests {
                diesel::delete(dsl::wasted_baggages)
                    .filter(dsl::id.eq(&request.baggage_id))
                    .filter(dsl::creator_id.eq(&request.account_id))
                    .execute(conn)?;
            }

            Ok(())
        })
    }
}

fn generate_wasted_baggage_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(23);
    result.push('w');

    parcel_common::rand::append_generate_string(&mut result, 22, CHARS);
    result
}
