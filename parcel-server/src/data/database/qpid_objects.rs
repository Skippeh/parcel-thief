use chrono::Utc;
use diesel::prelude::*;

use parcel_common::api_types::{
    self, object::ObjectType, requests::create_object::CreateObjectRequest,
};

use crate::db::{
    models::qpid_object::{
        bridge_info::{BridgeInfo, NewBridgeInfo},
        customize_info::{CustomizeInfo, NewCustomizeInfo},
        extra_info::{ExtraInfo, NewExtraInfo},
        parking_info::{NewParkingInfo, ParkingInfo},
        rope_info::{NewRopeInfo, RopeInfo},
        stone_info::{NewStoneInfo, StoneInfo},
        vehicle_info::{NewVehicleInfo, VehicleInfo},
        NewQpidObject, QpidObject,
    },
    QueryError,
};

use super::DatabaseConnection;

pub struct AddedQpidObject {
    pub object: QpidObject,
    pub rope_info: Option<RopeInfo>,
    pub stone_info: Option<StoneInfo>,
    pub bridge_info: Option<BridgeInfo>,
    pub parking_info: Option<ParkingInfo>,
    pub vehicle_info: Option<VehicleInfo>,
    pub extra_info: Option<ExtraInfo>,
    pub customize_info: Option<CustomizeInfo>,
    _phantom: std::marker::PhantomData<()>, // prevent other modules from creating this struct
}

impl AddedQpidObject {
    pub fn try_into_api_type(self) -> Result<api_types::object::Object, anyhow::Error> {
        let mut result = self.object.try_into_api_type()?;

        result.rope_info = self.rope_info.map(|i| i.into_api_type());
        result.stone_info = self.stone_info.map(|i| i.into_api_type());
        result.bridge_info = self.bridge_info.map(|i| i.into_api_type());
        result.parking_info = self.parking_info.map(|i| i.into_api_type());
        result.vehicle_info = self.vehicle_info.map(|i| i.into_api_type());
        result.extra_info = self.extra_info.map(|i| i.into_api_type());
        result.customize_info = self.customize_info.map(|i| i.into_api_type());

        Ok(result)
    }
}

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
    ) -> Result<AddedQpidObject, QueryError> {
        use crate::db::schema::qpid_objects::dsl;

        let conn = &mut *self.connection.get_pg_connection().await;
        let id = generate_object_id(request.object_type);
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
        let mut db_stone_info = None;
        let mut db_bridge_info = None;
        let mut db_parking_info = None;
        let mut db_vehicle_info = None;
        let mut db_extra_info = None;
        let mut db_customize_info = None;

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
            use crate::db::schema::qpid_object_stone_infos::table;
            db_stone_info = Some(
                diesel::insert_into(table)
                    .values(NewStoneInfo {
                        object_id: &id,
                        resting_count: stone_info.resting_count,
                    })
                    .get_result::<StoneInfo>(conn)?,
            )
        }

        if let Some(bridge_info) = &request.bridge_info {
            use crate::db::schema::qpid_object_bridge_infos::table;
            db_bridge_info = Some(
                diesel::insert_into(table)
                    .values(NewBridgeInfo {
                        object_id: &id,
                        angle: bridge_info.angle,
                    })
                    .get_result::<BridgeInfo>(conn)?,
            )
        }

        if let Some(parking_info) = &request.parking_info {
            use crate::db::schema::qpid_object_parking_infos::table;
            db_parking_info = Some(
                diesel::insert_into(table)
                    .values(NewParkingInfo {
                        object_id: &id,
                        location_id: parking_info.location_id,
                        dynamic_location_id: &parking_info.dynamic_location_id,
                        current_qpid_id: parking_info.current_qpid_id,
                        is_parking: parking_info.is_parking,
                    })
                    .get_result::<ParkingInfo>(conn)?,
            )
        }

        if let Some(vehicle_info) = &request.vehicle_info {
            use crate::db::schema::qpid_object_vehicle_infos::table;
            db_vehicle_info = Some(
                diesel::insert_into(table)
                    .values(NewVehicleInfo {
                        object_id: &id,
                        location_id: vehicle_info.location_id,
                        dynamic_location_id: &vehicle_info.dynamic_location_id,
                        current_qpid_id: vehicle_info.current_qpid_id,
                        is_parking: vehicle_info.is_parking,
                        is_lost: vehicle_info.is_lost,
                        is_race: vehicle_info.is_race,
                        customize_type: vehicle_info.customize_type,
                        customize_color: vehicle_info.customize_color,
                        new_pos_x: vehicle_info.new_position.0,
                        new_pos_y: vehicle_info.new_position.1,
                        new_pos_z: vehicle_info.new_position.2,
                        new_rot_x: vehicle_info.new_rotation.0,
                        new_rot_y: vehicle_info.new_rotation.1,
                        new_rot_z: vehicle_info.new_rotation.2,
                        exponent: vehicle_info.exponent,
                    })
                    .get_result::<VehicleInfo>(conn)?,
            )
        }

        if let Some(extra_info) = &request.extra_info {
            use crate::db::schema::qpid_object_extra_infos::table;
            db_extra_info = Some(
                diesel::insert_into(table)
                    .values(NewExtraInfo {
                        object_id: &id,
                        alternative_qpid_id: extra_info.alternative_qpid_id,
                    })
                    .get_result::<ExtraInfo>(conn)?,
            )
        }

        if let Some(customize_info) = &request.customize_info {
            use crate::db::schema::qpid_object_customize_infos::table;
            db_customize_info = Some(
                diesel::insert_into(table)
                    .values(NewCustomizeInfo {
                        object_id: &id,
                        customize_param: customize_info.customize_param as i32,
                        customize_color: customize_info.customize_color as i32,
                    })
                    .get_result::<CustomizeInfo>(conn)?,
            )
        }

        Ok(AddedQpidObject {
            object: db_object,
            rope_info: db_rope_info,
            stone_info: db_stone_info,
            bridge_info: db_bridge_info,
            parking_info: db_parking_info,
            vehicle_info: db_vehicle_info,
            extra_info: db_extra_info,
            customize_info: db_customize_info,
            _phantom: std::marker::PhantomData,
        })
    }
}

/// Generates a random 13 character long object id
/// The first character will always match the object type.
fn generate_object_id(obj_type: ObjectType) -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(13);
    let object_tag = serde_json::to_string(&obj_type).unwrap();
    result.push_str(object_tag.trim_matches('\"'));

    parcel_common::rand::append_generate_string(&mut result, 12, CHARS);
    result
}
