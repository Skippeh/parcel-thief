use std::collections::HashMap;

use anyhow::Result;
use http::{Request, Response};
use serde_json::Value;

use crate::proxy_response_handler::AuthResponse;

pub async fn log_gateway_request_and_response(
    request: (&Request<String>, Option<&String>),
    response: (&Response<String>, Option<&String>),
) -> Result<()> {
    let deserialized_request = match request.1 {
        None => None,
        Some(json) => Some(serde_json::from_str::<HashMap<String, Value>>(json)?),
    };
    let deserialized_response = match response.1 {
        None => None,
        Some(json) => Some(serde_json::from_str::<HashMap<String, Value>>(json)?),
    };

    let formatted_request = deserialized_request
        .map(|map| serde_json::to_string_pretty(&map))
        .unwrap_or_else(|| Ok("".into()))?;
    let formatted_response = deserialized_response
        .map(|map| serde_json::to_string_pretty(&map))
        .unwrap_or_else(|| Ok("".into()))?;

    println!(
        "{} {}:\n{}\n====\n{}:\n{}\n",
        request.0.method(),
        request.0.uri().path(),
        formatted_request,
        response.0.status(),
        formatted_response
    );

    Ok(())
}

pub async fn log_auth(request: &Request<String>, mut response: AuthResponse) -> Result<()> {
    response.session.token = "***".into();

    println!("AUTH:\n{:#?}\n====\n{:#?}\n", request, response);
    Ok(())
}
