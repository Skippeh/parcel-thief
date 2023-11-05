use std::collections::HashMap;

use chrono::{Days, Utc};
use diesel::{dsl::not, prelude::*};
use diesel_async::{
    scoped_futures::ScopedFutureExt, AsyncConnection, AsyncPgConnection, RunQueryDsl,
};
use parcel_common::api_types::{
    self,
    mission::{MissionType, OnlineMissionType, ProgressState},
    requests::{self},
    IntoDsApiType,
};

use crate::db::{
    models::mission::{
        baggage::{
            ammo_info::{AmmoInfo, NewAmmoInfo},
            Baggage, NewBaggage,
        },
        catapult_shell_info::{CatapultShellInfo, ChangeCatapultShellInfo, NewCatapultShellInfo},
        dynamic_location_info::{
            ChangeDynamicLocationInfo, DynamicLocationInfo, InfoType, NewDynamicLocationInfo,
        },
        dynamic_mission_info::{DynamicMissionInfo, NewDynamicMissionInfo},
        relation::{NewRelation, Relation},
        supply_info::{NewSupplyInfo, SupplyInfo},
        ChangeMission, Mission, NewMission,
    },
    schema::missions::dsl,
    QueryError,
};

use super::DatabaseConnection;

pub struct Missions<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> Missions<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn save_mission(
        &self,
        mission: &requests::add_missions::NewMission,
        owner_id: &str,
        custom_mission_id: Option<i64>,
    ) -> Result<DbMission, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            async move {
                let registered_time = Utc::now().naive_utc();
                let expiration_time = registered_time.checked_add_days(Days::new(14)).unwrap();
                let id = generate_mission_id();

                let db_mission = diesel::insert_into(dsl::missions)
                    .values(&NewMission {
                        id: &id,
                        area_id: mission.area_hash,
                        creator_id: owner_id,
                        worker_id: None,
                        qpid_id: mission.qpid_id,
                        qpid_start_location: mission.qpid_start_location,
                        qpid_end_location: mission.qpid_end_location,
                        qpid_delivered_location: None,
                        mission_static_id: mission.mission_static_id,
                        mission_type: mission.mission_type,
                        online_mission_type: mission.online_mission_type,
                        progress_state: ProgressState::Ready, // todo: this might be wrong
                        registered_time: &registered_time,
                        expiration_time: &expiration_time, // note: this isn't used yet (maybe it shouldn't be?)
                        custom_mission_id,
                    })
                    .get_result::<Mission>(conn)
                    .await?;

                let mut result = DbMission::from(db_mission);

                if let Some(supply_info) = &mission.supply_info {
                    use crate::db::schema::mission_supply_infos::table;
                    result.supply_info = Some(
                        diesel::insert_into(table)
                            .values(&NewSupplyInfo {
                                mission_id: &id,
                                item_hash: supply_info.item_hash,
                                amount: supply_info.amount,
                            })
                            .get_result(conn)
                            .await?,
                    );
                }

                {
                    async fn add_location_info(
                        conn: &mut AsyncPgConnection,
                        info_type: &api_types::mission::DynamicLocationInfo,
                        ty: InfoType,
                        mission_id: &str,
                    ) -> Result<DynamicLocationInfo, diesel::result::Error> {
                        use crate::db::schema::mission_dynamic_location_infos::table;
                        diesel::insert_into(table)
                            .values(&NewDynamicLocationInfo {
                                mission_id,
                                ty,
                                location_id: &info_type.location_object_id,
                                x: info_type.x,
                                y: info_type.y,
                                z: info_type.z,
                            })
                            .get_result(conn)
                            .await
                    }

                    if let Some(dynamic_start_info) = &mission.dynamic_start_info {
                        result.dynamic_start_info = Some(
                            add_location_info(conn, dynamic_start_info, InfoType::Start, &id)
                                .await?,
                        );
                    }

                    if let Some(dynamic_end_info) = &mission.dynamic_end_info {
                        result.dynamic_end_info = Some(
                            add_location_info(conn, dynamic_end_info, InfoType::End, &id).await?,
                        );
                    }

                    if let Some(dynamic_mission_info) = &mission.dynamic_mission_info {
                        use crate::db::schema::mission_dynamic_mission_infos::table;
                        result.dynamic_mission_info = Some(
                            diesel::insert_into(table)
                                .values(&NewDynamicMissionInfo {
                                    mission_id: &id,
                                    client_name_hash: dynamic_mission_info.client_name_hash,
                                    reward_name_hash: dynamic_mission_info.reward_name_hash,
                                })
                                .get_result(conn)
                                .await?,
                        );
                    }

                    if let Some(info) = &mission.catapult_shell_info {
                        use crate::db::schema::mission_catapult_shell_infos::table;
                        result.catapult_shell_info = Some(
                            diesel::insert_into(table)
                                .values(&NewCatapultShellInfo {
                                    mission_id: &id,
                                    local_id: info.local_id,
                                    x: info.x,
                                    y: info.y,
                                    z: info.z,
                                })
                                .get_result(conn)
                                .await?,
                        )
                    }
                }

                if let Some(baggages) = &mission.baggages {
                    use crate::db::schema::mission_baggages::table;
                    let mut db_baggages = Vec::with_capacity(baggages.len());
                    for baggage in baggages {
                        let db_baggage = diesel::insert_into(table)
                            .values(&NewBaggage {
                                mission_id: &id,
                                amount: baggage.amount,
                                name_hash: baggage.name_hash,
                                user_index: baggage.user_index,
                                x: baggage.x,
                                y: baggage.y,
                                z: baggage.z,
                                is_returned: baggage.is_returned,
                            })
                            .get_result::<Baggage>(conn)
                            .await?;
                        let baggage_id = db_baggage.id;

                        if let Some(ammo_info) = &baggage.ammo_info {
                            use crate::db::schema::mission_baggage_ammo_infos::table;
                            let ammo_info = diesel::insert_into(table)
                                .values(&NewAmmoInfo {
                                    baggage_id,
                                    ammo_id: &ammo_info.ammo_id,
                                    clip_count: ammo_info.clip_count,
                                    count: ammo_info.count,
                                })
                                .get_result::<AmmoInfo>(conn)
                                .await?;

                            result.baggage_ammo_infos.insert(baggage_id, ammo_info);
                        }

                        db_baggages.push(db_baggage);
                    }

                    result.baggages = db_baggages;
                }

                {
                    use crate::db::schema::mission_relations::dsl;
                    diesel::insert_into(dsl::mission_relations)
                        .values(&NewRelation {
                            mission_id: &id,
                            account_id: owner_id,
                            updated_at: &registered_time,
                        })
                        .execute(conn)
                        .await?;

                    result.relations.push(owner_id.to_owned());
                }

                Ok(result)
            }
            .scope_boxed()
        })
        .await
    }

    pub async fn delete_missions(
        &self,
        creator_id: &str,
        mission_ids: impl Iterator<Item = &str>,
    ) -> Result<usize, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        Ok(diesel::delete(
            dsl::missions
                .filter(dsl::creator_id.eq(creator_id))
                .filter(dsl::id.eq_any(mission_ids)),
        )
        .execute(conn)
        .await?)
    }

    pub async fn find_missions(
        &self,
        online_types: &[OnlineMissionType],
        mission_types: &[MissionType],
        exclude_accounts: &[&str],
        progress_states: &[ProgressState],
        qpid_ids: Option<&[i32]>,
    ) -> Result<Vec<Mission>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        let mut query = dsl::missions
            .filter(dsl::online_mission_type.eq_any(online_types))
            .filter(dsl::mission_type.eq_any(mission_types))
            .filter(dsl::progress_state.eq_any(progress_states))
            .filter(not(dsl::creator_id.eq_any(exclude_accounts)))
            .filter(dsl::custom_mission_id.is_null()) // exclude custom missions
            .into_boxed();

        if let Some(qpid_ids) = qpid_ids {
            query = query.filter(dsl::qpid_id.eq_any(qpid_ids));
        }

        Ok(query.get_results(conn).await?)
    }

    pub async fn get_ordered_missions(&self, account_id: &str) -> Result<Vec<Mission>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        let missions: Vec<Mission> = dsl::missions
            .filter(dsl::creator_id.eq(account_id))
            .order_by(dsl::registered_time.desc())
            .limit(1000)
            .get_results(conn)
            .await?;

        Ok(missions)
    }

    pub async fn query_mission_data(
        &self,
        missions: impl IntoIterator<Item = Mission>,
    ) -> Result<Vec<DbMission>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let mut db_missions = Vec::new();

        // todo: optimize this, use less db queries
        for mission in missions.into_iter() {
            let id = mission.id.clone();
            let mut mission: DbMission = mission.into();

            {
                use crate::db::schema::mission_supply_infos::dsl;
                mission.supply_info = dsl::mission_supply_infos
                    .filter(dsl::mission_id.eq(&id))
                    .first(conn)
                    .await
                    .optional()?;
            }

            {
                use crate::db::schema::mission_dynamic_location_infos::dsl;
                mission.dynamic_start_info = dsl::mission_dynamic_location_infos
                    .filter(dsl::mission_id.eq(&id))
                    .filter(dsl::type_.eq(InfoType::Start))
                    .first(conn)
                    .await
                    .optional()?;

                mission.dynamic_end_info = dsl::mission_dynamic_location_infos
                    .filter(dsl::mission_id.eq(&id))
                    .filter(dsl::type_.eq(InfoType::End))
                    .first(conn)
                    .await
                    .optional()?;

                mission.dynamic_delivered_info = dsl::mission_dynamic_location_infos
                    .filter(dsl::mission_id.eq(&id))
                    .filter(dsl::type_.eq(InfoType::Delivered))
                    .first(conn)
                    .await
                    .optional()?;
            }

            {
                use crate::db::schema::mission_dynamic_mission_infos::dsl;
                mission.dynamic_mission_info = dsl::mission_dynamic_mission_infos
                    .filter(dsl::mission_id.eq(&id))
                    .first(conn)
                    .await
                    .optional()?;
            }

            {
                use crate::db::schema::mission_catapult_shell_infos::dsl;
                mission.catapult_shell_info = dsl::mission_catapult_shell_infos
                    .filter(dsl::mission_id.eq(&id))
                    .first(conn)
                    .await
                    .optional()?;
            }

            {
                use crate::db::schema::mission_baggages::dsl;

                mission.baggages = dsl::mission_baggages
                    .filter(dsl::mission_id.eq(&id))
                    .get_results(conn)
                    .await?;

                for baggage in &mission.baggages {
                    use crate::db::schema::mission_baggage_ammo_infos::dsl;

                    let ammo_info = dsl::mission_baggage_ammo_infos
                        .filter(dsl::baggage_id.eq(baggage.id))
                        .first(conn)
                        .await
                        .optional()?;

                    if let Some(ammo_info) = ammo_info {
                        mission.baggage_ammo_infos.insert(baggage.id, ammo_info);
                    }
                }
            }

            {
                use crate::db::schema::mission_relations::dsl;

                mission.relations = dsl::mission_relations
                    .filter(dsl::mission_id.eq(&id))
                    .order_by(dsl::updated_at.asc())
                    .get_results::<Relation>(conn)
                    .await?
                    .into_iter()
                    .map(|rel| rel.account_id)
                    .collect();
            }

            db_missions.push(mission);
        }

        Ok(db_missions)
    }

    pub async fn get_by_id(&self, mission_id: &str) -> Result<Option<Mission>, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;
        let mission = dsl::missions
            .filter(dsl::id.eq(mission_id))
            .first(conn)
            .await
            .optional()?;

        Ok(mission)
    }

    pub async fn update_mission(
        &self,
        account_id: &str,
        mission_id: &str,
        data: &ChangeMission<'_>,
        baggages: Option<Option<&[api_types::mission::Baggage]>>,
        delivered_location_info: Option<Option<&api_types::mission::DynamicLocationInfo>>,
        catapult_shell_info: Option<Option<&api_types::mission::CatapultShellInfo>>,
    ) -> Result<(), QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
            async move {
                use dsl as dsl_missions;
                let affected_rows = diesel::update(dsl_missions::missions)
                    .filter(dsl::id.eq(mission_id))
                    .set(data)
                    .execute(conn)
                    .await?;

                if affected_rows == 0 {
                    return Ok(()); // should we return error instead if there's no mission with the specified id?
                }

                if let Some(info) = delivered_location_info {
                    use crate::db::schema::mission_dynamic_location_infos::dsl;
                    match info {
                        Some(info) => {
                            // upsert dynamic delivered location info
                            diesel::insert_into(dsl::mission_dynamic_location_infos)
                                .values(NewDynamicLocationInfo {
                                    mission_id,
                                    ty: InfoType::Delivered,
                                    location_id: &info.location_object_id,
                                    x: info.x,
                                    y: info.y,
                                    z: info.z,
                                })
                                .on_conflict((dsl::mission_id, dsl::type_))
                                .do_update()
                                .set(ChangeDynamicLocationInfo::from(info))
                                .execute(conn)
                                .await?;
                        }
                        None => {
                            // delete delivered location info
                            diesel::delete(dsl::mission_dynamic_location_infos)
                                .filter(dsl::mission_id.eq(mission_id))
                                .filter(dsl::type_.eq(InfoType::Delivered))
                                .execute(conn)
                                .await?;
                        }
                    }
                }

                if let Some(info) = catapult_shell_info {
                    use crate::db::schema::mission_catapult_shell_infos::dsl;
                    match info {
                        Some(info) => {
                            // upsert catapult shell info
                            diesel::insert_into(dsl::mission_catapult_shell_infos)
                                .values(NewCatapultShellInfo {
                                    mission_id,
                                    local_id: info.local_id,
                                    x: info.x,
                                    y: info.y,
                                    z: info.z,
                                })
                                .on_conflict(dsl::mission_id)
                                .do_update()
                                .set(ChangeCatapultShellInfo::from(info))
                                .execute(conn)
                                .await?;
                        }
                        None => {
                            // delete catapult shell info
                            diesel::delete(dsl::mission_catapult_shell_infos)
                                .filter(dsl::mission_id.eq(mission_id))
                                .execute(conn)
                                .await?;
                        }
                    }
                }

                if let Some(baggages) = baggages {
                    use crate::db::schema::mission_baggages::dsl;
                    // delete current baggages (this also deleted ammo infos since their baggage_id relation is ON DELETE CASCADE)
                    diesel::delete(dsl::mission_baggages)
                        .filter(dsl::mission_id.eq(mission_id))
                        .execute(conn)
                        .await?;

                    // insert new baggages if Some
                    if let Some(baggages) = baggages {
                        for baggage in baggages {
                            let db_baggage = diesel::insert_into(dsl::mission_baggages)
                                .values(NewBaggage {
                                    mission_id,
                                    amount: baggage.amount,
                                    name_hash: baggage.name_hash,
                                    user_index: baggage.user_index,
                                    x: baggage.x,
                                    y: baggage.y,
                                    z: baggage.z,
                                    is_returned: baggage.is_returned,
                                })
                                .get_result::<Baggage>(conn)
                                .await?;

                            if let Some(ammo_info) = &baggage.ammo_info {
                                use crate::db::schema::mission_baggage_ammo_infos::dsl;
                                diesel::insert_into(dsl::mission_baggage_ammo_infos)
                                    .values(NewAmmoInfo {
                                        baggage_id: db_baggage.id,
                                        ammo_id: &ammo_info.ammo_id,
                                        clip_count: ammo_info.clip_count,
                                        count: ammo_info.count,
                                    })
                                    .execute(conn)
                                    .await?;
                            }
                        }
                    }
                }

                // update relations
                {
                    use crate::db::schema::mission_relations::dsl;
                    diesel::insert_into(dsl::mission_relations)
                        .values(NewRelation {
                            mission_id,
                            account_id,
                            updated_at: &Utc::now().naive_utc(),
                        })
                        .on_conflict_do_nothing()
                        .execute(conn)
                        .await?;
                }

                Ok(())
            }
            .scope_boxed()
        })
        .await
    }
}

#[derive(Debug)]
pub struct DbMission {
    pub mission: Mission,
    pub supply_info: Option<SupplyInfo>,
    pub dynamic_start_info: Option<DynamicLocationInfo>,
    pub dynamic_end_info: Option<DynamicLocationInfo>,
    pub dynamic_delivered_info: Option<DynamicLocationInfo>,
    pub dynamic_mission_info: Option<DynamicMissionInfo>,
    pub catapult_shell_info: Option<CatapultShellInfo>,
    pub baggages: Vec<Baggage>,
    pub baggage_ammo_infos: HashMap<i64, AmmoInfo>,
    pub relations: Vec<String>,
}

impl From<Mission> for DbMission {
    fn from(value: Mission) -> Self {
        Self {
            mission: value,
            supply_info: None,
            dynamic_start_info: None,
            dynamic_end_info: None,
            dynamic_delivered_info: None,
            dynamic_mission_info: None,
            catapult_shell_info: None,
            baggages: Vec::default(),
            baggage_ammo_infos: HashMap::default(),
            relations: Vec::default(),
        }
    }
}

impl IntoDsApiType for DbMission {
    type ApiType = api_types::mission::Mission;

    fn into_ds_api_type(mut self) -> Self::ApiType {
        let mut api_mission = self.mission.into_ds_api_type();

        api_mission.supply_info = self.supply_info.map(|si| si.into_ds_api_type());
        api_mission.dynamic_start_info = self.dynamic_start_info.map(|d| d.into_ds_api_type());
        api_mission.dynamic_end_info = self.dynamic_end_info.map(|d| d.into_ds_api_type());
        api_mission.dynamic_delivered_info =
            self.dynamic_delivered_info.map(|d| d.into_ds_api_type());
        api_mission.dynamic_mission_info = self.dynamic_mission_info.map(|d| d.into_ds_api_type());
        api_mission.catapult_shell_info = self.catapult_shell_info.map(|c| c.into_ds_api_type());
        api_mission.baggages = self
            .baggages
            .into_iter()
            .map(|b| {
                let id = b.id;
                let mut baggage = b.into_ds_api_type();

                baggage.ammo_info = self
                    .baggage_ammo_infos
                    .remove(&id)
                    .map(|ammo| ammo.into_ds_api_type());
                baggage
            })
            .collect();
        api_mission.relations = self.relations;

        api_mission
    }
}

/// Generates a 23 character long mission id.
/// The first character will always be 'm'.
fn generate_mission_id() -> String {
    const CHARS: &[u8] = b"aAbBcCdDeEfFgGhHiIjJkKlLmMnNoOpPqQrRsStTuUvVwWxXyYzZ0123456789";
    let mut result = String::with_capacity(23);
    result.push('m');
    parcel_common::rand::append_generate_string(&mut result, 22, CHARS);

    result
}
