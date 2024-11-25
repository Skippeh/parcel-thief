use actix_web::{
    get,
    web::{Data, Json},
    HttpRequest,
};
use parcel_common::api_types::auth::{AuthResponse, SessionInfo, SessionProperties, UserInfo};

use crate::{
    data::database::Database,
    endpoints::{auth::infer_gateway_url, InternalError},
    session::Session,
    GatewayUrl,
};

/// This route doesn't exist on the "real" ds server, it's only used for debugging.
///
/// Leaving it in for production is safe since the same info is sent to the client on auth.
#[get("auth/me")]
pub async fn me(
    session: Session,
    database: Data<Database>,
    gateway_url: Data<Option<GatewayUrl>>,
    http_request: HttpRequest,
) -> Result<Json<AuthResponse>, InternalError> {
    let db = database.connect().await?;
    let accounts = db.accounts();
    let account = accounts
        .get_by_provider_id(session.provider, &session.provider_id)
        .await?
        .expect("Sessions can't exist without an account");

    let gateway_url = match gateway_url.as_ref() {
        Some(gateway_url) => gateway_url.0.clone(),
        None => {
            let url = infer_gateway_url(&http_request);

            log::debug!("Inferred gateway url: {}", url);

            url
        }
    };

    Ok(Json(AuthResponse {
        session: SessionInfo {
            gateway: gateway_url,
            token: session.get_token().to_owned(),
            properties: SessionProperties {
                last_login: account.last_login_date.and_utc().timestamp(),
            },
        },
        user: UserInfo {
            display_name: account.display_name,
            id: account.id,
            provider: account.provider,
        },
    }))
}
