use diesel::prelude::*;
use diesel::Connection;
use parcel_common::api_types::requests::get_wasted_baggages::WastedItem;

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

        conn.transaction(|conn| {
            diesel::insert_into(dsl::wasted_baggages)
                .values(
                    &items
                        .iter()
                        .map(|item| NewWastedBaggage {
                            id: generate_wasted_baggage_id(),
                            qpid_id,
                            creator_id: owner_id,
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
}

fn generate_wasted_baggage_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(23);
    result.push('w');

    parcel_common::rand::append_generate_string(&mut result, 22, CHARS);
    result
}
