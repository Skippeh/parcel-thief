use anyhow::{Context, Result};
use http::{Request, Response};
use parcel_common::{
    aes,
    api_types::{auth::AuthResponse, EncryptedData},
};

use crate::logger::{log_auth, log_gateway_request_and_response};

pub async fn handle_proxy_response(
    original_request: &Request<String>,
    response: &mut Response<String>,
) -> Result<()> {
    let uri_path = original_request.uri().path();

    if uri_path.eq_ignore_ascii_case("/auth/ds") {
        handle_auth_response_gateway(original_request, response)
            .await
            .context("could not rewrite gateway from auth response")?;
    } else if uri_path.to_lowercase().starts_with("/ds/e/") {
        match handle_gateway_action(original_request, response).await {
            Ok(_) => {}
            Err(err) => {
                log::error!("failed to record gateway action: {:?}", err);
                log::error!("{}\n{}", original_request.body(), response.body());
            }
        }
    }

    Ok(())
}

async fn handle_auth_response_gateway(
    original_request: &Request<String>,
    response: &mut Response<String>,
) -> Result<()> {
    // set gateway to the public url
    let json = response.body();

    let mut auth_response =
        serde_json::from_str::<AuthResponse>(json).context("could not deserialize json body")?;

    auth_response.session.gateway = crate::server::PUBLIC_URL.lock().await.clone();
    *response.body_mut() =
        serde_json::to_string(&auth_response).context("failed to serialize new json body")?;

    log_auth(original_request, auth_response)
        .await
        .context("failed to log")?;

    Ok(())
}

async fn handle_gateway_action(
    original_request: &Request<String>,
    response: &mut Response<String>,
) -> Result<()> {
    let request_json = match original_request.body().as_ref() {
        "" => None,
        json => Some(aes::decrypt_json_response(json).context("failed to decrypt request data")?),
    };

    let response_json = match response.body().as_ref() {
        "" => None,
        json => Some(aes::decrypt_json_response(json).context("failed to decrypt response data")?),
    };

    log_gateway_request_and_response(
        (original_request, request_json.as_ref().map(|d| d.as_ref())),
        (response, response_json.as_ref().map(|d| d.as_ref())),
    )
    .await
    .context("failed to log")?;

    Ok(())
}
