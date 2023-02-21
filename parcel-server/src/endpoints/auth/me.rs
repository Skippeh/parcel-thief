use actix_web::{
    get,
    web::{Data, Json},
};
use parcel_common::api_types::auth::{AuthResponse, SessionInfo, SessionProperties, UserInfo};

use crate::{data::database::Database, endpoints::InternalError, session::Session, GatewayUrl};

/// This route doesn't exist on the "real" ds server, it's only used for debugging.
///
/// Leaving it in for production is safe since the same info is sent to the client on auth.
#[get("auth/me")]
pub async fn me(
    session: Session,
    database: Data<Database>,
    gateway_url: Data<GatewayUrl>,
) -> Result<Json<AuthResponse>, InternalError> {
    let db = database.connect()?;
    let accounts = db.accounts();
    let account = accounts
        .get_by_provider_id(session.provider, &session.provider_id)
        .await?
        .unwrap(); // This is safe since a session can't exist without an account

    Ok(Json(AuthResponse {
        session: SessionInfo {
            gateway: gateway_url.as_ref().clone().into(),
            token: session.get_token().to_owned(),
            properties: SessionProperties {
                last_login: account.last_login_date.timestamp(),
            },
        },
        user: UserInfo {
            display_name: account.display_name,
            id: account.id,
            provider: account.provider,
        },
    }))
}
