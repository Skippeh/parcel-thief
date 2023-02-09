use std::collections::BTreeMap;

use anyhow::{Context, Result};
use chrono::Local;
use http::{Request, Response};
use parcel_common::api_types::auth::AuthResponse;
use serde::Serialize;
use serde_json::Value;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

#[derive(Serialize)]
struct LogData<'a> {
    request: &'a Option<BTreeMap<String, Value>>,
    response: &'a Option<BTreeMap<String, Value>>,
}

#[derive(Serialize)]
struct AuthLogData<'a> {
    path: &'a str,
    request_headers: BTreeMap<String, String>,
    response: &'a AuthResponse,
}

pub async fn log_gateway_request_and_response(
    request: (&Request<String>, Option<Option<&String>>),
    response: (&Response<String>, Option<Option<&String>>),
) -> Result<()> {
    let deserialized_request = match request.1 {
        Some(Some(json)) => Some(serde_json::from_str::<BTreeMap<String, Value>>(json)?),
        _ => None,
    };
    let deserialized_response = match response.1 {
        Some(Some(json)) => Some(serde_json::from_str::<BTreeMap<String, Value>>(json)?),
        _ => None,
    };

    println!("{} {}", request.0.method(), request.0.uri().path());

    let log_data = LogData {
        request: &deserialized_request,
        response: &deserialized_response,
    };
    save_request_log(&log_data, request.0).await?;

    Ok(())
}

pub async fn log_auth(request: &Request<String>, mut response: AuthResponse) -> Result<()> {
    response.session.token = "***".into();

    println!("{} {}", request.method(), request.uri().path());
    println!("AUTH: Authenticated as {:?}", response.user);

    let mut headers = BTreeMap::new();
    for header in request.headers() {
        headers.insert(
            header.0.to_string(),
            header.1.to_str().unwrap_or("INVALID").to_string(),
        );
    }

    let log_data = AuthLogData {
        path: &format!(
            "{}?{}",
            request.uri().path(),
            request.uri().query().unwrap_or_default()
        ),
        request_headers: headers,
        response: &response,
    };

    save_request_log(&log_data, request).await?;

    Ok(())
}

async fn save_request_log<T>(log_data: &T, request: &Request<String>) -> Result<(), anyhow::Error>
where
    T: Serialize,
{
    let file_contents =
        serde_json::to_string_pretty(&log_data).context("could not serialize log data")?;
    let (dir_path, file_name) = get_paths_for_request(request).await;
    std::fs::create_dir_all(&dir_path).context("could not create log folder")?;
    let mut writer = BufWriter::new(
        File::create(dir_path.join(file_name))
            .await
            .context("failed to create log file")?,
    );
    writer
        .write_all(file_contents.as_bytes())
        .await
        .context("could not write log contents to file")?;
    writer.flush().await?;
    Ok(())
}

async fn get_paths_for_request(request: &Request<String>) -> (std::path::PathBuf, String) {
    let logs_folder = crate::LOG_DIRECTORY.read().await.clone();
    let request_path = request.uri().path();
    let request_path = request_path.split_once('/').unwrap().1; // skip first / in request path
    let file_name = Local::now().format("%Y-%m-%d_%H-%M-%S.%f.json").to_string();
    let dir_path = logs_folder.join(request_path);
    (dir_path, file_name)
}
