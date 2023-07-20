use actix_web::{
    post,
    web::{Data, Json},
};
use flagset::FlagSet;
use parcel_common::api_types::frontend::auth::{AuthAccountInfo, LocalAuthRequest};

use crate::{
    data::{database::Database, hash_secret::HashSecret, jwt_secret::JwtSecret},
    frontend::{
        error::ApiError,
        result::{ApiResponse, ApiResult},
    },
};

#[post("auth/local")]
pub async fn auth_local(
    request: Json<LocalAuthRequest>,
    database: Data<Database>,
    jwt_secret: Data<JwtSecret>,
    hash_secret: Data<HashSecret>,
) -> ApiResult<AuthAccountInfo> {
    let conn = database.connect()?;
    let accounts = conn.frontend_accounts();

    let account = accounts
        .get_by_credentials(&request.username, &request.password, &hash_secret)
        .await?;

    match account {
        None => Err(ApiError::Unauthorized(anyhow::anyhow!(
            "The username or password is incorrect"
        ))),
        Some(account) => {
            // todo
            let permissions = FlagSet::new_truncated(account.permissions);
            let permissions_vec = permissions.into_iter().collect();
            let auth_token =
                super::create_auth_token(&account, &jwt_secret).map_err(anyhow::Error::msg)?;

            ApiResponse::ok(AuthAccountInfo {
                name: "todo".into(),
                avatar_url: None,
                game_account_id: account.game_account_id,
                auth_token,
                permissions: permissions_vec,
            })
        }
    }
}
