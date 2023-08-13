use actix_web::{
    get,
    web::{self, Data},
};
use parcel_common::api_types::{
    area::AreaHash,
    frontend::qpid_objects::{QpidObject, QpidObjectType},
};
use parcel_game_data::Area;

use crate::{
    data::database::Database,
    frontend::{
        error::ApiError,
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
};

#[get("qpidObjects/{area}")]
pub async fn list_qpid_objects(
    _session: JwtSession,
    database: Data<Database>,
    area: web::Path<Area>,
) -> ApiResult<Vec<QpidObject>> {
    let area = match area.into_inner() {
        Area::Area01 => Ok(AreaHash::EasternRegion),
        Area::Area02 => Ok(AreaHash::CentralRegion),
        Area::Area04 => Ok(AreaHash::WesternRegion),
        _ => Err(ApiError::Unprocessable(anyhow::anyhow!("Invalid area"))),
    }?;

    let conn = database.connect().await?;
    let qpid_objects = conn.qpid_objects().find_objects_by_area(area).await?;

    ApiResponse::ok(
        qpid_objects
            .into_iter()
            .map(|q| {
                let object_type = (q.object_type.clone(), q.sub_type.as_ref()).into();
                let unknown_type = match &object_type {
                    QpidObjectType::Unknown => {
                        Some((q.object_type.to_string(), q.sub_type.to_string()))
                    }
                    _ => None,
                };

                QpidObject {
                    id: q.id,
                    location: (
                        (q.pos_x as f64 / 100_000f64) as f32,
                        (q.pos_y as f64 / 100_000f64) as f32,
                        (q.pos_z as f64 / 100_000f64) as f32,
                    ),
                    object_type,
                    unknown_type,
                }
            })
            .collect(),
    )
}
