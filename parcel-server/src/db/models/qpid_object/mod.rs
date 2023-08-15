pub mod bridge_info;
pub mod comment;
pub mod construction_materials;
pub mod customize_info;
pub mod extra_info;
pub mod parking_info;
pub mod recycle_materials;
pub mod rope_info;
pub mod stone_info;
pub mod tag;
pub mod vehicle_info;

use std::num::TryFromIntError;

use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use parcel_common::api_types::{self, area::AreaHash, object::ObjectType, TryIntoDsApiType};

use crate::db::schema::qpid_objects;

#[derive(Debug, Queryable)]
pub struct QpidObject {
    pub id: String,
    pub creator_id: String,
    pub exponent: i32,
    pub likes: i64,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub rot_x: i32,
    pub rot_y: i32,
    pub rot_z: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub area_id: AreaHash,
    pub qpid_id: i32,
    pub sub_type: String,
    pub updated_time: NaiveDateTime,
    pub object_type: ObjectType,
    pub is_deleted: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_objects)]
pub struct NewQpidObject<'a> {
    pub id: &'a str,
    pub creator_id: &'a str,
    pub exponent: i32,
    pub likes: i64,
    pub pos_x: i32,
    pub pos_y: i32,
    pub pos_z: i32,
    pub rot_x: i32,
    pub rot_y: i32,
    pub rot_z: i32,
    pub grid_x: i32,
    pub grid_y: i32,
    pub area_id: AreaHash,
    pub qpid_id: i32,
    pub object_type: &'a ObjectType,
    pub sub_type: &'a str,
    pub updated_time: &'a NaiveDateTime,
}

impl TryIntoDsApiType for QpidObject {
    type ApiType = api_types::object::Object;
    type Error = TryFromIntError;

    /// Converts self to api equivalent type. Note that all relational columns are set to None.
    ///
    /// Fails if likes is >u32::MAX (db value is stored as i64 due to postgres lacking unsigned types)
    fn try_into_ds_api_type(self) -> Result<Self::ApiType, Self::Error> {
        Ok(Self::ApiType {
            creator_account_id: self.creator_id,
            exponent: self.exponent,
            object_id: self.id,
            position: (self.pos_x, self.pos_y, self.pos_z),
            likes: self.likes.try_into()?,
            map_index: (self.grid_x, self.grid_y, self.area_id),
            qpid_id: self.qpid_id,
            rotation: (self.rot_x, self.rot_y, self.rot_z),
            sub_type: self.sub_type,
            object_type: self.object_type,
            updated_time: self.updated_time.timestamp_millis(),
            construction_materials_contributions: None,
            recycle_materials_contributions: None,
            priority: false,
            baggages: None,
            comments: None,
            rope_info: None,
            stone_info: None,
            bridge_info: None,
            parking_info: None,
            vehicle_info: None,
            extra_info: None,
            customize_info: None,
            tags: None,
        })
    }
}
