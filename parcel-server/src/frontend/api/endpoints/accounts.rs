use actix_web::{
    get, post, put,
    web::{Data, Json, Path, Query},
};
use flagset::FlagSet;
use parcel_common::api_types::frontend::{
    accounts::{
        CreateCredentialsRequest, FrontendAccount as ApiFrontendAccount, FrontendAccountListItem,
        GameAccountListItem, ListAccountsResponse, ListAccountsType, LocalAccount,
        ProviderConnection, ResetPasswordRequest, SetAccountPermissionsRequest,
    },
    auth::FrontendPermissions,
};
use serde::Deserialize;

use crate::{
    data::{database::Database, hash_secret::HashSecret},
    endpoints::{EmptyResponse, ValidatedJson},
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
            let account_names = frontend_accounts
                .get_display_names(&data_accounts.iter().collect::<Vec<_>>())
                .await?;

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

            ApiResponse::ok(ListAccountsResponse::Frontend { accounts: result })
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

            ApiResponse::ok(ListAccountsResponse::Game { accounts: result })
        }
    }
}

#[get("accounts/frontend/{id}")]
pub async fn get_frontend_account(
    session: JwtSession,
    database: Data<Database>,
    params: Path<i64>,
) -> ApiResult<ApiFrontendAccount> {
    let account_id = params.into_inner();

    // Make sure the current session is either looking up their own account or has permissions to manage accounts
    if account_id != session.account_id
        && !session.has_permissions(FrontendPermissions::ManageAccounts)
    {
        return Err(ApiError::Forbidden);
    }

    let conn = database.connect()?;
    let accounts = conn.frontend_accounts();
    let account = accounts.get_by_id(account_id).await?;
    let credentials = accounts.get_credentials(account_id).await?;
    let provider_connection = accounts.get_provider_connection(account_id).await?;

    match account {
        None => Err(ApiError::NotFound),
        Some(account) => {
            let permissions = FlagSet::new_truncated(account.permissions)
                .into_iter()
                .collect();

            let name = accounts
                .get_display_names(&[&account])
                .await?
                .into_iter()
                .next()
                .map(|kv| kv.1);

            ApiResponse::ok(ApiFrontendAccount {
                id: account.id,
                game_id: account.game_account_id,
                permissions,
                local_account: credentials.map(|c| LocalAccount {
                    username: c.username,
                }),
                provider_connection: provider_connection.map(|c| ProviderConnection {
                    provider: c.provider,
                    provider_id: c.provider_id,
                    name,
                }),
            })
        }
    }
}

#[put("accounts/frontend/{id}/permissions")]
pub async fn set_account_permissions(
    session: JwtSession,
    params: Path<i64>,
    request: Json<SetAccountPermissionsRequest>,
    database: Data<Database>,
) -> ApiResult<Vec<FrontendPermissions>> {
    // Check that we have permission
    if !session.has_permissions(FrontendPermissions::ManageAccounts) {
        return Err(ApiError::Forbidden);
    }

    let account_id = params.into_inner();

    // Check that we're not modifying our own permissions
    if account_id == session.account_id
        && !request
            .permissions
            .contains(&FrontendPermissions::ManageAccounts)
    {
        return Err(ApiError::Unprocessable(anyhow::anyhow!(
            "You cannot remove the 'Manage accounts' permission from your own account"
        )));
    }

    let conn = database.connect()?;
    let accounts = conn.frontend_accounts();

    let mut new_permissions = FlagSet::default();

    for permission in &request.permissions {
        new_permissions |= *permission;
    }

    let set_permissions = accounts
        .set_permissions(account_id, new_permissions)
        .await?;

    ApiResponse::ok(set_permissions.into_iter().collect())
}

#[post("accounts/createCredentials/{id}")]
pub async fn create_credentials(
    session: JwtSession,
    database: Data<Database>,
    request: ValidatedJson<CreateCredentialsRequest>,
    params: Path<i64>,
    hash_secret: Data<HashSecret>,
) -> ApiResult<LocalAccount> {
    let account_id = params.into_inner();

    // Check that we have permission
    if account_id != session.account_id
        && !session.has_permissions(FrontendPermissions::ManageAccounts)
    {
        return Err(ApiError::Forbidden);
    }

    let conn = database.connect()?;
    let accounts = conn.frontend_accounts();
    let account = accounts.get_by_id(account_id).await?;

    // Verify that the account exists
    match account {
        None => return Err(ApiError::NotFound),
        Some(_account) => {
            // Verify that the username isn't taken
            if accounts.username_exists(&request.username).await? {
                return Err(ApiError::validation_errors(&[(
                    "username",
                    "usernameExists",
                )]));
            }

            let credentials = accounts
                .create_credentials(
                    account_id,
                    &request.username,
                    &request.password,
                    hash_secret.as_ref(),
                )
                .await?;

            ApiResponse::ok(LocalAccount {
                username: credentials.username,
            })
        }
    }
}

#[post("accounts/resetPassword/{id}")]
pub async fn reset_password(
    session: JwtSession,
    params: Path<i64>,
    request: Json<ResetPasswordRequest>,
    database: Data<Database>,
    hash_secret: Data<HashSecret>,
) -> ApiResult<EmptyResponse> {
    let account_id = params.into_inner();

    Err(anyhow::anyhow!("Not implemented").into())
}
