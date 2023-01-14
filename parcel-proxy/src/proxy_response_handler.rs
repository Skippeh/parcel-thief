use anyhow::{Context, Result};
use http::{Request, Response};
use serde::{Deserialize, Serialize};

use crate::logger::log_request_and_response;

// todo: move these to a common library project?

#[derive(Deserialize, Serialize, Debug)]
struct UserInfo {
    provider: String,
    id: String,
    display_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SessionProperties {
    /// The epoch time in seconds of the last login (seems to always be same as the current time)
    #[serde(rename = "ll")]
    last_login: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SessionInfo {
    token: String,
    gateway: String,
    properties: SessionProperties,
}

#[derive(Deserialize, Serialize, Debug)]
struct AuthResponse {
    user: UserInfo,
    session: SessionInfo,
}

pub async fn handle_proxy_response(
    original_request: &Request<String>,
    response: &mut Response<String>,
) -> Result<()> {
    let uri_path = original_request.uri().path();

    if uri_path.eq_ignore_ascii_case("/auth/ds") {
        rewrite_auth_response_gateway(response)
            .await
            .context("could not rewrite gateway from auth response")?;
    } else if uri_path.to_lowercase().starts_with("/ds/e/") {
        // todo: decrypt data
    }

    log_request_and_response(original_request, response)
        .await
        .context("failed to log")?;

    Ok(())
}

async fn rewrite_auth_response_gateway(response: &mut Response<String>) -> Result<()> {
    let json = response.body();

    let mut auth_response =
        serde_json::from_str::<AuthResponse>(json).context("could not deserialize json body")?;

    auth_response.session.gateway = crate::PUBLIC_URL.lock().await.clone();

    println!("{auth_response:#?}");

    *response.body_mut() =
        serde_json::to_string(&auth_response).context("failed to serialize new json body")?;

    Ok(())
}
