use chrono::Utc;
use diesel::{dsl::not, prelude::*};

use parcel_common::api_types::{
    self, area::AreaHash, object::ObjectType, requests::create_object::CreateObjectRequest,
};

use crate::db::{
    models::qpid_object::{
        bridge_info::{BridgeInfo, NewBridgeInfo},
        comment::{Comment, NewComment, NewPhrase, Phrase},
        construction_materials::{
            ChangeConstructionMaterials, ConstructionMaterials, NewConstructionMaterials,
        },
        customize_info::{ChangeCustomizeInfo, CustomizeInfo, NewCustomizeInfo},
        extra_info::{ChangeExtraInfo, ExtraInfo, NewExtraInfo},
        parking_info::{ChangeParkingInfo, NewParkingInfo, ParkingInfo},
        recycle_materials::{ChangeRecycleMaterials, NewRecycleMaterials, RecycleMaterials},
        rope_info::{NewRopeInfo, RopeInfo},
        stone_info::{ChangeStoneInfo, NewStoneInfo, StoneInfo},
        tag::{NewTag, Tag},
        vehicle_info::{ChangeVehicleInfo, NewVehicleInfo, VehicleInfo},
        NewQpidObject, QpidObject,
    },
    QueryError,
};

use super::DatabaseConnection;

#[derive(Debug)]
pub struct DbQpidObject {
    pub object: QpidObject,
    pub rope_info: Option<RopeInfo>,
    pub stone_info: Option<StoneInfo>,
    pub bridge_info: Option<BridgeInfo>,
    pub parking_info: Option<ParkingInfo>,
    pub vehicle_info: Option<VehicleInfo>,
    pub extra_info: Option<ExtraInfo>,
    pub customize_info: Option<CustomizeInfo>,
    pub comment: Option<Comment>,
    pub comment_phrases: Option<Vec<Phrase>>,
    pub construction_materials: Option<Vec<ConstructionMaterials>>,
    pub recycle_materials: Option<Vec<RecycleMaterials>>,
    pub tags: Option<Vec<Tag>>,
}

impl DbQpidObject {
    pub fn try_into_api_type(self) -> Result<api_types::object::Object, anyhow::Error> {
        let mut result = self.object.try_into_api_type()?;

        result.rope_info = self.rope_info.map(|i| i.into_api_type());
        result.stone_info = self.stone_info.map(|i| i.into_api_type());
        result.bridge_info = self.bridge_info.map(|i| i.into_api_type());
        result.parking_info = self.parking_info.map(|i| i.into_api_type());
        result.vehicle_info = self.vehicle_info.map(|i| i.into_api_type());
        result.extra_info = self.extra_info.map(|i| i.into_api_type());
        result.customize_info = self.customize_info.map(|i| i.into_api_type());
        result.construction_materials_contributions = self
            .construction_materials
            .map(|i| i.into_iter().map(|mats| mats.into_api_type()).collect());
        result.recycle_materials_contributions = self
            .recycle_materials
            .map(|i| i.into_iter().map(|mats| mats.into_api_type()).collect());
        result.tags = self.tags.map(|i| i.into_iter().map(|t| t.tag).collect());

        if let Some(comment) = self.comment {
            let mut comments = Vec::with_capacity(1);
            let mut comment = comment.try_into_api_type()?;

            if let Some(mut phrases) = self.comment_phrases {
                phrases.sort_unstable_by(|a, b| a.sort_order.cmp(&b.sort_order));
                comment.phrases = phrases.into_iter().map(|p| p.into_api_type()).collect();
            } else {
                anyhow::bail!("No phrases specified in comment");
            }

            comments.push(comment);
            result.comments = Some(comments);
        }

        Ok(result)
    }
}

pub enum ChangeInfo<'a> {
    Stone(&'a ChangeStoneInfo),
    Parking(&'a ChangeParkingInfo<'a>),
    Vehicle(&'a ChangeVehicleInfo<'a>),
    Customize(&'a ChangeCustomizeInfo),
    Extra(&'a ChangeExtraInfo),
    // bridge and comment is intentionally excluded
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
    ) -> Result<DbQpidObject, QueryError> {
        use crate::db::schema::qpid_objects::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            let id = generate_object_id(&request.object_type);
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
                object_type: &request.object_type,
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
            let mut db_comment = None;
            let mut db_comment_phrases = None;

            if let Some(comment) = &request.comment {
                use crate::db::schema::qpid_object_comments::table;
                db_comment = Some(
                    diesel::insert_into(table)
                        .values(NewComment {
                            object_id: &id,
                            writer: creator_id,
                            likes: comment.likes as i64,
                            parent_index: comment.parent_index as i16,
                            is_deleted: comment.is_deleted,
                            reference_object: &comment.reference_object,
                        })
                        .get_result::<Comment>(conn)?,
                );

                let comment_id = db_comment.as_ref().unwrap().id;
                let mut phrases = Vec::new();

                for (index, phrase) in comment.phrases.iter().enumerate() {
                    use crate::db::schema::qpid_object_comment_phrases::table;
                    let db_phrase = diesel::insert_into(table)
                        .values(NewPhrase {
                            comment_id,
                            phrase: *phrase,
                            sort_order: index as i16,
                        })
                        .get_result::<Phrase>(conn)?;

                    phrases.push(db_phrase);
                }

                db_comment_phrases = Some(phrases);
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
                            new_pos_x: vehicle_info.new_position.map(|pos| pos.0),
                            new_pos_y: vehicle_info.new_position.map(|pos| pos.1),
                            new_pos_z: vehicle_info.new_position.map(|pos| pos.2),
                            new_rot_x: vehicle_info.new_rotation.map(|rot| rot.0),
                            new_rot_y: vehicle_info.new_rotation.map(|rot| rot.1),
                            new_rot_z: vehicle_info.new_rotation.map(|rot| rot.2),
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

            Ok(DbQpidObject {
                object: db_object,
                rope_info: db_rope_info,
                stone_info: db_stone_info,
                bridge_info: db_bridge_info,
                parking_info: db_parking_info,
                vehicle_info: db_vehicle_info,
                extra_info: db_extra_info,
                customize_info: db_customize_info,
                comment: db_comment,
                comment_phrases: db_comment_phrases,
                construction_materials: None,
                recycle_materials: None,
                tags: None,
            })
        })
    }

    pub async fn get_by_id(&self, object_id: &str) -> Result<Option<QpidObject>, QueryError> {
        use crate::db::schema::qpid_objects::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        dsl::qpid_objects
            .find(object_id)
            .get_result(conn)
            .optional()
            .map_err(|err| err.into())
    }

    pub async fn mark_deleted_for_account(
        &self,
        object_id: &str,
        account_id: &str,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            use crate::db::schema::qpid_objects::dsl;
            let object = dsl::qpid_objects
                .filter(dsl::id.eq(object_id))
                .first::<QpidObject>(conn)?;
            self.add_tag(conn, object_id, &format!("del_{}", account_id))?;

            if object.object_type == ObjectType::Sign {
                use crate::db::schema::qpid_object_comments::dsl;

                // set is_deleted on related comments to true
                diesel::update(dsl::qpid_object_comments)
                    .filter(dsl::object_id.eq(object_id))
                    .filter(dsl::writer.eq(account_id))
                    .filter(dsl::is_deleted.eq(false))
                    .set(dsl::is_deleted.eq(true))
                    .execute(conn)?;
            }

            Ok(())
        })
    }

    pub async fn update_info(
        &self,
        object_id: &str,
        info: ChangeInfo<'_>,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        match info {
            ChangeInfo::Stone(info) => {
                use crate::db::schema::qpid_object_stone_infos::dsl;
                diesel::update(dsl::qpid_object_stone_infos)
                    .filter(dsl::object_id.eq(object_id))
                    .set(info)
                    .execute(conn)?;
            }
            ChangeInfo::Parking(info) => {
                use crate::db::schema::qpid_object_parking_infos::dsl;
                diesel::update(dsl::qpid_object_parking_infos)
                    .filter(dsl::object_id.eq(object_id))
                    .set(info)
                    .execute(conn)?;
            }
            ChangeInfo::Vehicle(info) => {
                use crate::db::schema::qpid_object_vehicle_infos::dsl;
                diesel::update(dsl::qpid_object_vehicle_infos)
                    .filter(dsl::object_id.eq(object_id))
                    .set(info)
                    .execute(conn)?;
            }
            ChangeInfo::Customize(info) => {
                use crate::db::schema::qpid_object_customize_infos::dsl;
                diesel::update(dsl::qpid_object_customize_infos)
                    .filter(dsl::object_id.eq(object_id))
                    .set(info)
                    .execute(conn)?;
            }
            ChangeInfo::Extra(info) => {
                use crate::db::schema::qpid_object_extra_infos::dsl;
                diesel::update(dsl::qpid_object_extra_infos)
                    .filter(dsl::object_id.eq(object_id))
                    .set(info)
                    .execute(conn)?;
            }
        }

        Ok(())
    }

    pub async fn find_objects(
        &self,
        area_hashes: &[AreaHash],
        qpid_ids: &[i32],
        priority_ids: Option<&[&str]>,
        limit: i64,
        exclude_account_ids: &[&str],
    ) -> Result<Vec<QpidObject>, QueryError> {
        use crate::db::schema::qpid_objects::dsl;

        if limit <= 0 {
            return Ok(Vec::new());
        }

        let conn = &mut *self.connection.get_pg_connection().await;

        let mut result_objects = Vec::with_capacity(limit as usize);
        let priority_ids = priority_ids.unwrap_or_default();

        // Find objects created by accounts in priority_ids
        if !priority_ids.is_empty() {
            let objects = dsl::qpid_objects
                .filter(dsl::qpid_id.eq_any(qpid_ids))
                .filter(dsl::area_id.eq_any(area_hashes))
                .filter(dsl::creator_id.eq_any(priority_ids))
                .filter(not(dsl::creator_id.eq_any(exclude_account_ids)))
                .limit(limit)
                .get_results::<QpidObject>(conn)?;

            result_objects.extend(objects);
        }

        // If we're not at the limit yet, find missions not created by accounts in priority_ids
        if result_objects.len() < limit as usize {
            let objects = dsl::qpid_objects
                .filter(dsl::qpid_id.eq_any(qpid_ids))
                .filter(dsl::area_id.eq_any(area_hashes))
                .filter(not(dsl::creator_id.eq_any(priority_ids)))
                .filter(not(dsl::creator_id.eq_any(exclude_account_ids)))
                .limit(limit - result_objects.len() as i64)
                .get_results::<QpidObject>(conn)?;

            result_objects.extend(objects);
        }

        result_objects.shrink_to_fit();
        Ok(result_objects)
    }

    pub async fn find_objects_by_id(&self, ids: &[String]) -> Result<Vec<QpidObject>, QueryError> {
        use crate::db::schema::qpid_objects::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let objects = dsl::qpid_objects
            .filter(dsl::id.eq_any(ids))
            .get_results(conn)?;

        Ok(objects)
    }

    pub async fn query_object_data(
        &self,
        objects: impl IntoIterator<Item = QpidObject>,
    ) -> Result<Vec<DbQpidObject>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let mut result = Vec::new();

        // todo: optimize this (use less queries)

        for object in objects {
            let mut db_object = DbQpidObject {
                object,
                rope_info: None,
                stone_info: None,
                bridge_info: None,
                parking_info: None,
                vehicle_info: None,
                extra_info: None,
                customize_info: None,
                comment: None,
                comment_phrases: None,
                construction_materials: None,
                recycle_materials: None,
                tags: None,
            };
            let id = &db_object.object.id;

            {
                use crate::db::schema::qpid_object_rope_infos::dsl;

                db_object.rope_info = dsl::qpid_object_rope_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_stone_infos::dsl;

                db_object.stone_info = dsl::qpid_object_stone_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_bridge_infos::dsl;

                db_object.bridge_info = dsl::qpid_object_bridge_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_parking_infos::dsl;

                db_object.parking_info = dsl::qpid_object_parking_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_vehicle_infos::dsl;

                db_object.vehicle_info = dsl::qpid_object_vehicle_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_extra_infos::dsl;

                db_object.extra_info = dsl::qpid_object_extra_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_customize_infos::dsl;

                db_object.customize_info = dsl::qpid_object_customize_infos
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;
            }

            {
                use crate::db::schema::qpid_object_comments::dsl;

                db_object.comment = dsl::qpid_object_comments
                    .filter(dsl::object_id.eq(id))
                    .first(conn)
                    .optional()?;

                if let Some(comment) = &db_object.comment {
                    use crate::db::schema::qpid_object_comment_phrases::dsl;

                    db_object.comment_phrases = Some(
                        dsl::qpid_object_comment_phrases
                            .filter(dsl::comment_id.eq(&comment.id))
                            .get_results::<Phrase>(conn)?,
                    );
                }
            }

            {
                use crate::db::schema::qpid_object_construction_materials::dsl;

                db_object.construction_materials = Some(
                    dsl::qpid_object_construction_materials
                        .filter(dsl::object_id.eq(id))
                        .get_results::<ConstructionMaterials>(conn)?,
                )
            }

            {
                use crate::db::schema::qpid_object_recycle_materials::dsl;

                db_object.recycle_materials = Some(
                    dsl::qpid_object_recycle_materials
                        .filter(dsl::object_id.eq(id))
                        .get_results::<RecycleMaterials>(conn)?,
                )
            }

            {
                use crate::db::schema::qpid_object_tags::dsl;

                db_object.tags = Some(
                    dsl::qpid_object_tags
                        .filter(dsl::object_id.eq(id))
                        .get_results::<Tag>(conn)?,
                )
            }

            result.push(db_object);
        }

        Ok(result)
    }

    pub async fn add_remove_tag_from_objects(
        &self,
        tag: &str,
        add: Option<impl Iterator<Item = &str>>,
        delete: Option<impl Iterator<Item = &str>>,
    ) -> Result<(), QueryError> {
        use crate::db::schema::qpid_object_tags::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            // add tag to objects
            if let Some(add) = add {
                diesel::insert_into(dsl::qpid_object_tags)
                    .values(
                        &add.map(|id| NewTag { object_id: id, tag })
                            .collect::<Vec<_>>(),
                    )
                    .on_conflict_do_nothing()
                    .execute(conn)?;
            }

            // remove tag from objects
            if let Some(delete) = delete {
                diesel::delete(dsl::qpid_object_tags)
                    .filter(dsl::tag.eq(tag))
                    .filter(dsl::object_id.eq_any(delete))
                    .execute(conn)?;
            }

            Ok(())
        })
    }

    pub async fn contribute_construction_materials(
        &self,
        contributor_id: Option<&str>,
        object_id: &str,
        upgrade_materials: &[i32; 6],
        repair_materials: &[i32; 6],
    ) -> Result<(), QueryError> {
        use crate::db::schema::qpid_object_construction_materials::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let new_row = NewConstructionMaterials {
            object_id,
            contributor_id,
            contribute_time: &chrono::Utc::now().naive_utc(),
            mats_0: upgrade_materials[0],
            mats_1: upgrade_materials[1],
            mats_2: upgrade_materials[2],
            mats_3: upgrade_materials[3],
            mats_4: upgrade_materials[4],
            mats_5: upgrade_materials[5],
            repair_0: repair_materials[0],
            repair_1: repair_materials[1],
            repair_2: repair_materials[2],
            repair_3: repair_materials[3],
            repair_4: repair_materials[4],
            repair_5: repair_materials[5],
        };

        diesel::insert_into(dsl::qpid_object_construction_materials)
            .values(&new_row)
            .on_conflict((dsl::object_id, dsl::contributor_id))
            .do_update()
            .set(ChangeConstructionMaterials::from(&new_row))
            .execute(conn)?;

        Ok(())
    }

    pub async fn contribute_recycle_materials(
        &self,
        contributor_id: Option<&str>,
        object_id: &str,
        materials: &[i32; 6],
    ) -> Result<(), QueryError> {
        use crate::db::schema::qpid_object_recycle_materials::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let new_row = NewRecycleMaterials {
            object_id,
            contributor_id,
            mats_0: materials[0],
            mats_1: materials[1],
            mats_2: materials[2],
            mats_3: materials[3],
            mats_4: materials[4],
            mats_5: materials[5],
            recycle_time: &chrono::Utc::now().naive_utc(),
        };

        diesel::insert_into(dsl::qpid_object_recycle_materials)
            .values(&new_row)
            .on_conflict((dsl::object_id, dsl::contributor_id))
            .do_update()
            .set(ChangeRecycleMaterials::from(&new_row))
            .execute(conn)?;

        Ok(())
    }

    fn add_tag(
        &self,
        conn: &mut PgConnection,
        object_id: &str,
        tag: &str,
    ) -> Result<(), QueryError> {
        use crate::db::schema::qpid_object_tags::table;
        diesel::insert_into(table)
            .values(NewTag { object_id, tag })
            .execute(conn)?;

        Ok(())
    }
}

/// Generates a 13 character long object id.
/// The first character will always match the object type.
fn generate_object_id(obj_type: &ObjectType) -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(13);
    let object_tag = serde_json::to_string(&obj_type).unwrap();
    let object_tag = object_tag.trim_matches('\"');
    result.push(
        object_tag
            .chars()
            .next()
            .expect("Object tag is never empty"),
    );

    parcel_common::rand::append_generate_string(&mut result, 12, CHARS);
    result
}
