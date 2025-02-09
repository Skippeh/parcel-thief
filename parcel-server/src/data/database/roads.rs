use std::collections::HashMap;

use base64::Engine;
use diesel::{dsl::not, prelude::*};
use diesel_async::{scoped_futures::ScopedFutureExt, AsyncConnection, RunQueryDsl};
use parcel_common::api_types::{
    requests::{create_road::CreateRoadRequest, find_qpid_objects::RoadRequest},
    IntoDsApiType,
};

use crate::db::{
    models::road::{NewRoad, NewRoadData, NewRoadViaQpid, Road, RoadData, RoadViaQpid},
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

impl IntoDsApiType for DbRoad {
    type ApiType = parcel_common::api_types::road::Road;

    fn into_ds_api_type(self) -> Self::ApiType {
        let via_qpids = if !self.via_qpids.is_empty() {
            Some(self.via_qpids.into_iter().map(|vq| vq.qpid_id).collect())
        } else {
            None
        };

        Self::ApiType {
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
            async move {
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
                    .get_result::<Road>(conn)
                    .await?;

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
                        .get_results::<RoadViaQpid>(conn)
                        .await?;

                    result.via_qpids.extend(db_via_qpids.into_iter());
                }

                {
                    use crate::db::schema::road_data::dsl;
                    let road_data =
                        base64::engine::general_purpose::STANDARD.decode(&request.data)?;

                    diesel::insert_into(dsl::road_data)
                        .values(&NewRoadData {
                            road_id: &result.road.id,
                            data: &road_data,
                        })
                        .execute(conn)
                        .await?;
                }

                Ok(result)
            }
            .scope_boxed()
        })
        .await
    }

    pub async fn find_roads(
        &self,
        parameters: &RoadRequest,
        exclude_ids: &[&str],
        _priority_ids: Option<&[&str]>,
    ) -> Result<Vec<DbRoad>, QueryError> {
        use crate::db::schema::roads::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;
        let mut result = Vec::new();

        // todo: respect priority_ids and parameters.prioritized_location_id

        let mut roads_query = dsl::roads
            .filter(dsl::qpid_end_id.eq_any(&parameters.end_qpids))
            .filter(dsl::data_version.eq(parameters.data_version))
            .filter(not(dsl::creator_id.eq_any(exclude_ids)))
            .order_by(dsl::created_at.desc())
            .limit(parameters.count as i64)
            .into_boxed();

        if let Some(required_location_id) = &parameters.required_location_id {
            roads_query = roads_query
                .filter(dsl::location_start_id.eq(required_location_id))
                .or_filter(dsl::location_end_id.eq(required_location_id));
        }

        let roads: Vec<Road> = roads_query.get_results::<Road>(conn).await?;

        {
            use crate::db::schema::road_via_qpids::dsl;
            let road_ids = roads.iter().map(|r| &r.id).collect::<Vec<_>>();

            let all_via_qpids: Vec<RoadViaQpid> = dsl::road_via_qpids
                .filter(dsl::road_id.eq_any(road_ids))
                .order_by(dsl::sort_order.asc())
                .get_results::<RoadViaQpid>(conn)
                .await?;

            let mut via_qpids = HashMap::<String, Vec<RoadViaQpid>>::new();
            for via_qpid in all_via_qpids {
                let vec = via_qpids
                    .entry(via_qpid.road_id.clone())
                    .or_insert_with(Vec::new);
                vec.push(via_qpid);
            }

            roads.into_iter().for_each(|road| {
                let db_road = DbRoad {
                    via_qpids: via_qpids.remove(&road.id).unwrap_or_default(),
                    road,
                };

                result.push(db_road);
            })
        }

        Ok(result)
    }

    pub async fn get_road_data(&self, road_id: &str) -> Result<Option<RoadData>, QueryError> {
        use crate::db::schema::road_data::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        Ok(dsl::road_data.find(road_id).first(conn).await.optional()?)
    }
}

fn generate_road_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(23);
    result.push('o');
    parcel_common::rand::append_generate_string(&mut result, 22, CHARS);

    result
}
