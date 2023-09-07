use actix_web::{
    post,
    web::{Data, Json},
};
use chrono::{DateTime, NaiveDateTime, Utc};
use parcel_common::api_types::requests::get_highway_resources::{
    ConstructionContributors, Contributor, GetHighwayResourcesRequest, GetHighwayResourcesResponse,
    PutResource,
};

use crate::{data::database::Database, endpoints::InternalError, session::Session};

#[post("getHighwayResources")]
pub async fn get_highway_resources(
    request: Json<GetHighwayResourcesRequest>,
    session: Session,
    database: Data<Database>,
) -> Result<Json<GetHighwayResourcesResponse>, InternalError> {
    let conn = database.connect().await?;
    let highway_resources = conn.highway_resources();
    let likes = conn.likes();

    let mut devoted_resources = highway_resources
        .get_contributors(
            request
                .constructions
                .iter()
                .map(|id_since| {
                    Ok((
                        id_since.construction_id,
                        parse_micro_date_time(id_since.last_login_date)?,
                    ))
                })
                .collect::<Result<Vec<_>, anyhow::Error>>()?,
            &request.resource_ids,
            &session.account_id,
            None,
        )
        .await?
        .into_iter()
        .map(|(construction_id, contributors)| ConstructionContributors {
            construction_id,
            contributors: contributors
                .into_iter()
                .map(|account_id| Contributor {
                    account_id,
                    likes: 0,
                })
                .collect(),
        })
        .collect::<Vec<ConstructionContributors>>();

    let mut account_ids = devoted_resources
        .iter()
        .flat_map(|res| res.contributors.iter().map(|con| &con.account_id))
        .collect::<Vec<_>>();

    account_ids.push(&session.account_id);

    // Query total likes and update contributor likes
    let total_likes = likes
        .get_total_highway_likes(account_ids.iter().map(|id| id as &str))
        .await?;

    for devoted_resource in &mut devoted_resources {
        for contributor in &mut devoted_resource.contributors {
            contributor.likes = *total_likes.get(&contributor.account_id).unwrap_or(&0);
        }
    }

    let total_resources = highway_resources
        .get_total_resources(
            request
                .constructions
                .iter()
                .map(|id_since| id_since.construction_id),
            request.resource_ids.iter().copied(),
        )
        .await?
        .into_iter()
        .map(|resources| PutResource {
            construction_id: resources.construction_id,
            resource_id: resources.resource_id,
            put_num: resources.num_resources,
            users_put_num: 0, // This is always 0 it seems
        })
        .collect();

    Ok(Json(GetHighwayResourcesResponse {
        construction_contributors: devoted_resources,
        put_resources: total_resources,
        users_like: *total_likes.get(&session.account_id).unwrap_or(&0),
    }))
}

fn parse_micro_date_time(mut date: i64) -> Result<DateTime<Utc>, anyhow::Error> {
    date -= 62135596800000000; // epoch expressed in microseconds

    let date_time = NaiveDateTime::from_timestamp_micros(date)
        .ok_or_else(|| anyhow::anyhow!("Date out of range"))?
        .and_utc();
    Ok(date_time)
}
