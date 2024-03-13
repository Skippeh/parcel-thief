use diesel_async::RunQueryDsl;

use crate::db::{
    models::custom_mission::{
        CustomMission, CustomMissionReward, CustomMissionType, NewCustomMission,
        NewCustomMissionReward,
    },
    QueryError,
};

use super::DatabaseConnection;

pub struct CustomMissions<'db> {
    connection: &'db DatabaseConnection<'db>,
}

impl<'db> CustomMissions<'db> {
    pub fn new(connection: &'db DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn create_mission(
        &self,
        ty: CustomMissionType,
        creator_id: i64,
    ) -> Result<CustomMission, QueryError> {
        use crate::db::schema::custom_missions::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let mission = diesel::insert_into(dsl::custom_missions)
            .values(&NewCustomMission {
                creator_id,
                ty,
                created_at: None,
            })
            .get_result(conn)
            .await?;

        Ok(mission)
    }

    pub async fn create_reward(
        &self,
        custom_mission_id: i64,
        item_hash: i32,
        amount: i32,
    ) -> Result<CustomMissionReward, QueryError> {
        use crate::db::schema::custom_mission_rewards::dsl;
        let conn = &mut *self.connection.get_pg_connection().await;

        let reward = diesel::insert_into(dsl::custom_mission_rewards)
            .values(&NewCustomMissionReward {
                custom_mission_id,
                item_hash,
                amount,
            })
            .get_result(conn)
            .await?;

        Ok(reward)
    }
}
