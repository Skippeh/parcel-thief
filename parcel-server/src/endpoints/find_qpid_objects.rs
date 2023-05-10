use actix_web::{
    post,
    web::{Data, Json},
};
use parcel_common::api_types::{
    object::QpidObjectsResponse,
    requests::find_qpid_objects::{FindQpidObjectsRequest, FindQpidObjectsResponse},
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("findQpidObjects")]
pub async fn find_qpid_objects(
    request: Json<FindQpidObjectsRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<FindQpidObjectsResponse>, InternalError> {
    let conn = database.connect()?;
    let objects = conn.qpid_objects();
    let request = request.into_inner();
    let area_hash = request.area_hash;
    let qpid_id = request.qpid_id;
    let mut result = FindQpidObjectsResponse {
        normal: QpidObjectsResponse::default(),
    };

    if let Some(mission) = request.mission {
        // This has never been used in any of my tests, so maybe it's an old thing that was replaced by the findMissions endpoint?
        log::warn!(
            "Expected FindQpidObjectsRequest.mission to be None, but found: {:#?}",
            mission
        );
    }

    let mut priority_ids = None;

    if let Some(req_priority_ids) = &request.account_ids {
        let mut ids = Vec::with_capacity(req_priority_ids.len());

        for id in req_priority_ids {
            ids.push(id.as_ref());
        }

        priority_ids = Some(ids);
    }

    if let Some(_object) = request.object {
        let found_objects = objects
            .find_objects(
                &[area_hash],
                &[qpid_id],
                priority_ids.as_deref(),
                10000,
                &[&session.account_id],
            )
            .await?;

        let api_objects = objects
            .query_object_data(found_objects)
            .await?
            .into_iter()
            .map(|obj| obj.try_into_api_type())
            .collect::<Result<_, _>>()?;

        result.normal.object_p = Some(api_objects);
    }

    if let Some(mut road_request) = request.road {
        road_request.count = 100;

        if let Some(-1) = &road_request.required_location_id {
            road_request.required_location_id = None;
        }

        let roads = conn.roads();

        let found_roads = roads
            .find_roads(
                &road_request,
                &[&session.account_id],
                priority_ids.as_deref(),
            )
            .await?
            .into_iter()
            .map(|r| r.into_api_type())
            .collect();

        result.normal.roads = Some(found_roads);
    }

    Ok(Json(result))
}
