use chrono::{Days, Utc};
use diesel::prelude::*;
use parcel_common::api_types::{self, mission::ProgressState, requests};

use crate::db::{
    models::mission::{
        baggage::{
            ammo_info::{AmmoInfo, NewAmmoInfo},
            Baggage, NewBaggage,
        },
        dynamic_location_info::{DynamicLocationInfo, InfoType, NewDynamicLocationInfo},
        dynamic_mission_info::{DynamicMissionInfo, NewDynamicMissionInfo},
        supply_info::{NewSupplyInfo, SupplyInfo},
        Mission, NewMission,
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
    ) -> Result<DbMission, QueryError> {
        let conn = &mut *self.connection.get_pg_connection().await;

        conn.transaction(|conn| {
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
                    expiration_time: &expiration_time,
                })
                .get_result(conn)?;

            let mut result = DbMission {
                mission: db_mission,
                supply_info: None,
                dynamic_start_info: None,
                dynamic_end_info: None,
                dynamic_delivered_info: None,
                dynamic_mission_info: None,
                baggages: Vec::default(),
            };

            if let Some(supply_info) = &mission.supply_info {
                use crate::db::schema::mission_supply_infos::table;
                result.supply_info = Some(
                    diesel::insert_into(table)
                        .values(&NewSupplyInfo {
                            mission_id: &id,
                            item_hash: supply_info.item_hash,
                            amount: supply_info.amount,
                        })
                        .get_result(conn)?,
                );
            }

            {
                fn add_location_info(
                    conn: &mut PgConnection,
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
                }

                if let Some(dynamic_start_info) = &mission.dynamic_start_info {
                    result.dynamic_start_info = Some(add_location_info(
                        conn,
                        dynamic_start_info,
                        InfoType::Start,
                        &id,
                    )?);
                }

                if let Some(dynamic_end_info) = &mission.dynamic_end_info {
                    result.dynamic_end_info = Some(add_location_info(
                        conn,
                        dynamic_end_info,
                        InfoType::End,
                        &id,
                    )?);
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
                            .get_result(conn)?,
                    );
                }
            }

            if let Some(baggages) = &mission.baggages {
                use crate::db::schema::mission_baggages::table;
                let mut res_baggages = Vec::with_capacity(baggages.len());
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
                        .get_result::<Baggage>(conn)?;
                    let baggage_id = db_baggage.id;
                    let mut res_baggage = db_baggage.into_api_type();

                    if let Some(ammo_info) = &baggage.ammo_info {
                        use crate::db::schema::mission_baggage_ammo_infos::table;
                        res_baggage.ammo_info = Some(
                            diesel::insert_into(table)
                                .values(&NewAmmoInfo {
                                    baggage_id,
                                    ammo_id: &ammo_info.ammo_id,
                                    clip_count: ammo_info.clip_count,
                                    count: ammo_info.count,
                                })
                                .get_result::<AmmoInfo>(conn)?
                                .into_api_type(),
                        );
                    }

                    res_baggages.push(res_baggage);
                }
            }

            Ok(result)
        })
    }
}

pub struct DbMission {
    pub mission: Mission,
    pub supply_info: Option<SupplyInfo>,
    pub dynamic_start_info: Option<DynamicLocationInfo>,
    pub dynamic_end_info: Option<DynamicLocationInfo>,
    pub dynamic_delivered_info: Option<DynamicLocationInfo>,
    pub dynamic_mission_info: Option<DynamicMissionInfo>,
    pub baggages: Vec<Baggage>,
}

impl DbMission {
    pub fn into_api_type(self) -> api_types::mission::Mission {
        let mut api_mission = self.mission.into_api_type();

        api_mission.supply_info = self.supply_info.map(|si| si.into_api_type());
        api_mission.dynamic_start_info = self.dynamic_start_info.map(|d| d.into_api_type());
        api_mission.dynamic_end_info = self.dynamic_end_info.map(|d| d.into_api_type());
        api_mission.dynamic_delivered_info = self.dynamic_delivered_info.map(|d| d.into_api_type());
        api_mission.dynamic_mission_info = self.dynamic_mission_info.map(|d| d.into_api_type());
        api_mission.baggages = self
            .baggages
            .into_iter()
            .map(|b| b.into_api_type())
            .collect();

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
