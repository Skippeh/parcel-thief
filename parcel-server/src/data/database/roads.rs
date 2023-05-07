use base64::Engine;
use diesel::prelude::*;
use parcel_common::api_types::requests::create_road::CreateRoadRequest;

use crate::db::{
    models::road::{NewRoad, NewRoadData, NewRoadViaQpid, Road, RoadViaQpid},
    QueryError,
};

use super::DatabaseConnection;

pub struct DbRoad {
    pub road: Road,
    pub via_qpids: Vec<RoadViaQpid>,
}

#[derive(Debug, thiserror::Error)]
pub enum CreateRoadError {
    #[error("Database error: {0}")]
    QueryError(QueryError),

    #[error("Invalid base64 road data")]
    InvalidRoadData,
}

impl From<diesel::result::Error> for CreateRoadError {
    fn from(value: diesel::result::Error) -> Self {
        CreateRoadError::QueryError(QueryError::QueryError(value))
    }
}

impl From<base64::DecodeError> for CreateRoadError {
    fn from(_: base64::DecodeError) -> Self {
        CreateRoadError::InvalidRoadData
    }
}

impl DbRoad {
    pub fn into_api_type(self) -> parcel_common::api_types::road::Road {
        let via_qpids = if !self.via_qpids.is_empty() {
            Some(self.via_qpids.into_iter().map(|vq| vq.qpid_id).collect())
        } else {
            None
        };

        parcel_common::api_types::road::Road {
            area_hash: self.road.area_hash,
            creator_account_id: self.road.creator_id,
            start_location_id: self.road.location_start_id,
            end_location_id: self.road.location_end_id,
            start_qpid_id: self.road.qpid_start_id,
            end_qpid_id: self.road.qpid_end_id,
            max_height_difference: self.road.max_height_difference,
            online_id: self.road.id,
            path_length: self.road.length,
            created_time: self.road.created_at.timestamp_millis(),
            data_version: self.road.data_version,
            via_qpids,
        }
    }
}

pub struct Roads<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> Roads<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn create_road_from_request(
        &self,
        account_id: &str,
        request: &CreateRoadRequest,
    ) -> Result<DbRoad, CreateRoadError> {
        use crate::db::schema::roads::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            let road = diesel::insert_into(dsl::roads)
                .values(&NewRoad {
                    id: &generate_road_id(),
                    area_hash: request.area_hash,
                    creator_id: account_id,
                    qpid_start_id: request.start_qpid_id,
                    qpid_end_id: request.end_qpid_id,
                    location_start_id: request.start_location_id,
                    location_end_id: request.end_location_id,
                    max_height_difference: request.max_height_difference,
                    length: request.path_length,
                    created_at: &chrono::Utc::now().naive_utc(),
                    data_version: request.data_version,
                })
                .get_result::<Road>(conn)?;

            let mut result = DbRoad {
                road,
                via_qpids: Vec::with_capacity(
                    request.via_qpids.as_ref().map(|vq| vq.len()).unwrap_or(0),
                ),
            };

            if let Some(via_qpids) = &request.via_qpids {
                use crate::db::schema::road_via_qpids::dsl;

                let db_via_qpids = diesel::insert_into(dsl::road_via_qpids)
                    .values(
                        &via_qpids
                            .iter()
                            .enumerate()
                            .map(|(index, qpid_id)| NewRoadViaQpid {
                                road_id: &result.road.id,
                                qpid_id: *qpid_id,
                                sort_order: index as i32,
                            })
                            .collect::<Vec<_>>(),
                    )
                    .get_results::<RoadViaQpid>(conn)?;

                result.via_qpids.extend(db_via_qpids.into_iter());
            }

            {
                use crate::db::schema::road_data::dsl;
                let road_data = base64::engine::general_purpose::STANDARD.decode(&request.data)?;

                diesel::insert_into(dsl::road_data)
                    .values(&NewRoadData {
                        road_id: &result.road.id,
                        data: &road_data,
                    })
                    .execute(conn)?;
            }

            Ok(result)
        })
    }
}

fn generate_road_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(23);
    result.push('o');
    parcel_common::rand::append_generate_string(&mut result, 22, CHARS);

    result
}
