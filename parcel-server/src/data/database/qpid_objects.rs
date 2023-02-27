use chrono::Utc;
use diesel::prelude::*;

use parcel_common::api_types::requests::create_object::CreateObjectRequest;

use crate::db::{
    models::qpid_object::{
        rope_info::{NewRopeInfo, RopeInfo},
        NewQpidObject, QpidObject,
    },
    QueryError,
};

use super::DatabaseConnection;

pub struct QpidObjects<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> QpidObjects<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn create_from_request(
        &self,
        request: &CreateObjectRequest,
        creator_id: &str,
    ) -> Result<QpidObject, QueryError> {
        use crate::db::schema::qpid_objects::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let id = generate_object_id();
        let now = Utc::now().naive_utc();

        let qpid_object = NewQpidObject {
            id: &id,
            creator_id,
            exponent: request.exponent,
            likes: request.likes as i64,
            pos_x: request.pos_x,
            pos_y: request.pos_y,
            pos_z: request.pos_z,
            rot_x: request.rot_x,
            rot_y: request.rot_y,
            rot_z: request.rot_z,
            grid_x: request.grid_x,
            grid_y: request.grid_y,
            area_id: request.area_hash,
            qpid_id: request.qpid_id,
            object_type: request.object_type,
            sub_type: &request.sub_type,
            updated_time: &now,
        };

        let db_object = diesel::insert_into(dsl::qpid_objects)
            .values(qpid_object)
            .get_result::<QpidObject>(conn)?;
        let mut db_rope_info = None;

        // todo: insert relational data if any
        if let Some(comment) = &request.comment {
            // As far as i can tell comments are some unfinished or unused feature.
            // (Signs use object_type and sub_type to define their type)
            log::warn!("Ignoring comment data on new object: {:#?}", comment);
            panic!("Expected comment to be None"); // todo: replace by returning error
        }

        if let Some(rope_info) = &request.rope_info {
            use crate::db::schema::qpid_object_rope_infos::table;
            db_rope_info = Some(
                diesel::insert_into(table)
                    .values(NewRopeInfo {
                        object_id: &id,
                        pitch: rope_info.pitch,
                        heading: rope_info.heading,
                        len: rope_info.length,
                    })
                    .get_result::<RopeInfo>(conn)?,
            );
        }

        if let Some(stone_info) = &request.stone_info {
            todo!()
        }

        if let Some(bridge_info) = &request.bridge_info {
            todo!()
        }

        if let Some(parking_info) = &request.parking_info {
            todo!()
        }

        if let Some(vehicle_info) = &request.vehicle_info {
            todo!()
        }

        if let Some(extra_info) = &request.extra_info {
            todo!()
        }

        if let Some(customize_info) = &request.customize_info {
            todo!()
        }

        Ok(db_object)
    }
}

/// Generates a random 13 character long object id
fn generate_object_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    parcel_common::rand::generate_string(13, CHARS)
}
