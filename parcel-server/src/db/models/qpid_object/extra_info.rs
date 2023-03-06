use diesel::{AsChangeset, Insertable, Queryable};
use parcel_common::api_types;

use crate::db::schema::qpid_object_extra_infos;

#[derive(Debug, Queryable)]
pub struct ExtraInfo {
    pub object_id: String,
    pub alternative_qpid_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = qpid_object_extra_infos, primary_key(object_id))]
pub struct NewExtraInfo<'a> {
    pub object_id: &'a str,
    pub alternative_qpid_id: i32,
}

impl ExtraInfo {
    pub fn into_api_type(self) -> api_types::object::ExtraInfo {
        api_types::object::ExtraInfo {
            alternative_qpid_id: self.alternative_qpid_id,
        }
    }
}

#[derive(Debug, AsChangeset)]
#[diesel(table_name = qpid_object_extra_infos)]
pub struct ChangeExtraInfo {
    pub alternative_qpid_id: i32,
}

impl From<&api_types::object::ExtraInfo> for ChangeExtraInfo {
    fn from(value: &api_types::object::ExtraInfo) -> Self {
        Self {
            alternative_qpid_id: value.alternative_qpid_id,
        }
    }
}
