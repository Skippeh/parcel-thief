use actix_web::{
    post,
    web::{Data, Json},
};
use flagset::FlagSet;
use parcel_common::api_types::{
    auth::Provider,
    frontend::auth::{AuthAccountInfo, LocalAuthRequest},
};

use crate::{
    data::{
        database::Database, hash_secret::HashSecret, jwt_secret::JwtSecret, platforms::steam::Steam,
    },
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
    steam: Data<Steam>,
) -> ApiResult<AuthAccountInfo> {
    let conn = database.connect().await?;
    let accounts = conn.frontend_accounts();

    let account = accounts
        .get_by_credentials(&request.username, &request.password, &hash_secret)
        .await?;

    match account {
        None => Err(ApiError::Unauthorized(anyhow::anyhow!(
            "The username or password is incorrect"
        ))),
        Some(account) => {
            let permissions = FlagSet::new_truncated(account.permissions);
            let permissions_vec = permissions.into_iter().collect();
            let (auth_token, expire_date) =
                super::create_auth_token(&account, &jwt_secret).map_err(anyhow::Error::msg)?;

            accounts
                .add_session(account.id, &auth_token, &expire_date.naive_utc())
                .await?;

            let name = accounts
                .get_display_names(&[&account])
                .await?
                .into_iter()
                .next()
                .map(|(_, name)| name)
                .unwrap_or_else(|| "".to_string());

            let avatar_url = {
                let provider_connection = accounts.get_provider_connection(account.id).await?;

                match provider_connection {
                    Some(provider_connection) => match provider_connection.provider {
                        Provider::Steam => {
                            let steam_id = provider_connection
                                .provider_id
                                .parse::<u64>()
                                .map_err(anyhow::Error::msg)?;
                            let user_summary = steam
                                .get_player_summaries(&[&steam_id])
                                .await?
                                .into_iter()
                                .next();

                            match user_summary {
                                Some((_, summary)) => Some(summary.avatar_full),
                                None => None,
                            }
                        }
                        Provider::Epic => None,
                        Provider::Server => None,
                    },
                    None => None,
                }
            };

            ApiResponse::ok(AuthAccountInfo {
                name,
                avatar_url,
                game_account_id: account.game_account_id,
                auth_token,
                permissions: permissions_vec,
            })
        }
    }
}
