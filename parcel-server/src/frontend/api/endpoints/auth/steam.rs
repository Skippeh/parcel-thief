use actix_web::{
    get,
    web::{Data, Redirect},
    HttpRequest,
};
use anyhow::Context;
use diesel_async::scoped_futures::ScopedFutureExt;
use flagset::FlagSet;
use parcel_common::api_types::{
    auth::Provider,
    frontend::auth::{AuthAccountInfo, CheckAuthResponse},
};
use steam_auth::Verifier;

use crate::{
    data::{database::Database, jwt_secret::JwtSecret, platforms::steam::Steam},
    db::models::frontend_account::{NewAccountProviderConnection, NewFrontendAccount},
    frontend::{api::endpoints::auth::FrontendAuthCache, error::ApiError},
    ServerSettings,
};

use super::generate_response_token;

#[get("auth/callback/steam")]
pub async fn steam_callback(
    request: HttpRequest,
    database: Data<Database>,
    steam: Data<Steam>,
    auth_cache: Data<FrontendAuthCache>,
    jwt_secret: Data<JwtSecret>,
    server_settings: Data<ServerSettings>,
) -> Result<Redirect, ApiError> {
    let (request, verifier) =
        Verifier::from_querystring(&request.query_string()).map_err(anyhow::Error::msg)?;

    let (parts, body) = request.into_parts();

    let client = reqwest::Client::new();
    let response = client
        .post(&parts.uri.to_string())
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .context("Failed to send request to verify steam callback")?;

    let text = response
        .text()
        .await
        .context("Failed to read steam callback response")?;

    let response_token = generate_response_token();
    let response = match verifier.verify_response(text) {
        Ok(steam_id) => {
            let conn = database.connect().await?;
            let accounts = conn.frontend_accounts();

            let account = accounts
                .get_by_provider(Provider::Steam, &steam_id.to_string())
                .await?;

            match account {
                Some(account) => {
                    create_auth_response(&jwt_secret, &steam, &accounts, account, steam_id).await?
                }
                None => {
                    if !server_settings.read().await.allow_frontend_login {
                        CheckAuthResponse::Failure {
                            error: "No frontend account was found for this Steam account".into(),
                        }
                    } else {
                        let account = conn
                            .transaction(|conn| {
                                async {
                                    // check if there's a game account for this steam account
                                    let game_account = conn
                                        .accounts()
                                        .get_by_provider_id(Provider::Steam, &steam_id.to_string())
                                        .await?;

                                    match game_account {
                                        None => Ok(None),
                                        Some(game_account) => {
                                            let accounts = conn.frontend_accounts();

                                            // create frontend account
                                            let account = accounts
                                                .add_account(&NewFrontendAccount {
                                                    game_account_id: Some(&game_account.id),
                                                    created_at: None,
                                                    permissions: 0,
                                                })
                                                .await?;

                                            // attach provider connection
                                            accounts
                                                .add_provider_connection(
                                                    &NewAccountProviderConnection {
                                                        account_id: account.id,
                                                        created_at: None,
                                                        provider: Provider::Steam,
                                                        provider_id: &steam_id.to_string(),
                                                    },
                                                )
                                                .await?;

                                            Ok(Some(account))
                                        }
                                    }
                                }
                                .scope_boxed()
                            })
                            .await?;

                        match account {
                            None => CheckAuthResponse::Failure {
                                error: "Game account not found, log in to the game server and try again".into(),
                            },
                            Some(account) => {
                                create_auth_response(&jwt_secret, &steam, &accounts, account, steam_id).await?
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            log::error!("Failed to verify steam callback: {}", e);

            CheckAuthResponse::Failure {
                error: "Failed to verify authentication".into(),
            }
        }
    };

    auth_cache.insert(response_token.clone(), response).await;

    Ok(Redirect::to(format!(
        "/frontend/login?callback_token={}",
        response_token
    )))
}

async fn create_auth_response(
    jwt_secret: &JwtSecret,
    steam: &Steam,
    accounts: &crate::data::database::frontend_accounts::FrontendAccounts<'_>,
    account: crate::db::models::frontend_account::FrontendAccount,
    steam_id: u64,
) -> Result<CheckAuthResponse, anyhow::Error> {
    let permissions = FlagSet::new_truncated(account.permissions);
    let permissions_vec = permissions.into_iter().collect();
    let (auth_token, expire_date) =
        super::create_auth_token(&account, &jwt_secret).map_err(anyhow::Error::msg)?;

    accounts
        .add_session(account.id, &auth_token, &expire_date.naive_utc())
        .await?;

    let user_summary = steam
        .get_player_summaries(&[&steam_id])
        .await?
        .remove(&steam_id)
        .context("Failed to get user summary")?;

    Ok(CheckAuthResponse::Success(AuthAccountInfo {
        auth_token,
        avatar_url: Some(user_summary.avatar_full),
        name: user_summary.name,
        game_account_id: account.game_account_id,
        permissions: permissions_vec,
    }))
}
