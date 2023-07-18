use std::collections::HashMap;

use actix_web::{
    get,
    web::{Data, Query},
};
use flagset::FlagSet;
use parcel_common::api_types::frontend::{
    accounts::{
        FrontendAccountListItem, GameAccountListItem, ListAccountsResponse, ListAccountsType,
    },
    auth::FrontendPermissions,
};
use serde::Deserialize;

use crate::{
    data::database::{Database, DatabaseConnection},
    db::models::frontend_account::FrontendAccount,
    frontend::{
        error::ApiError,
        jwt_session::JwtSession,
        result::{ApiResponse, ApiResult},
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListAccountsQuery {
    accounts_type: ListAccountsType,
}

#[get("accounts")]
pub async fn list_accounts(
    session: JwtSession,
    db: Data<Database>,
    query: Query<ListAccountsQuery>,
) -> ApiResult<ListAccountsResponse> {
    // check that the session has access
    if !session.has_permissions(FrontendPermissions::ManageAccounts) {
        return Err(ApiError::Forbidden);
    }

    let conn = db.connect()?;

    match query.accounts_type {
        ListAccountsType::Frontend => {
            let frontend_accounts = conn.frontend_accounts();
            let mut result = Vec::new();

            let data_accounts = frontend_accounts.get_all().await?;
            let account_names = get_frontend_account_names(conn, &data_accounts).await?;

            for account in data_accounts {
                let name = account_names
                    .get(&account.id)
                    .map(|n| n.to_owned())
                    .unwrap_or_else(|| "".to_string());

                let permissions = FlagSet::new_truncated(account.permissions)
                    .into_iter()
                    .collect();

                result.push(FrontendAccountListItem {
                    id: account.id,
                    game_id: account.game_account_id,
                    name,
                    permissions,
                });
            }

            ApiResponse::ok(ListAccountsResponse::FrontendAccounts { accounts: result })
        }
        ListAccountsType::Game => {
            let accounts = conn.accounts();
            let mut result = Vec::new();
            let data_accounts = accounts.get_all().await?;

            for account in data_accounts {
                result.push(GameAccountListItem {
                    game_id: account.id,
                    name: account.display_name,
                    provider: account.provider,
                    provider_id: account.provider_id,
                    last_login: account.last_login_date.and_utc().to_rfc3339(),
                });
            }

            ApiResponse::ok(ListAccountsResponse::GameAccounts { accounts: result })
        }
    }
}

/// Query the names for the specified frontend accounts.
///
/// * Accounts with a game account id will use their provider/in-game names
/// * Accounts without a game account id will use their login names (if any)
/// * Accounts without a game account id or login names will not be added to the returned hash map
async fn get_frontend_account_names(
    conn: DatabaseConnection<'_>,
    accounts: &[FrontendAccount],
) -> Result<HashMap<i64, String>, ApiError> {
    let game_accounts = conn.accounts();

    // Get the account ids where the game account id is Some
    let game_account_ids = accounts
        .iter()
        .filter(|account| account.game_account_id.is_some())
        .map(|account| {
            (
                account
                    .game_account_id
                    .as_ref()
                    .expect("Game account id should always be Some"),
                account.id,
            )
        })
        .collect::<HashMap<_, _>>();

    let credential_account_ids = accounts
        .iter()
        .filter(|account| account.game_account_id.is_none())
        .map(|account| account.id)
        .collect::<Vec<_>>();

    // Query names from game accounts
    let game_account_names = game_accounts
        .get_by_ids(&game_account_ids.keys().collect::<Vec<_>>())
        .await?
        .into_iter()
        .map(|acc| {
            (
                *game_account_ids
                    .get(&acc.id)
                    .expect("Game account ids should always contain the account id"),
                acc.display_name.clone(),
            )
        })
        .collect::<HashMap<_, _>>();

    // Query usernames from accounts without a game account id
    let frontend_accounts = conn.frontend_accounts();
    let usernames = frontend_accounts
        .get_login_usernames(&credential_account_ids)
        .await?;

    let mut result = HashMap::new();
    result.extend(game_account_names.into_iter());
    result.extend(usernames.into_iter());

    Ok(result)
}
