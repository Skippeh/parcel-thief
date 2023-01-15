use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result};
use chrono::Local;
use http::{Request, Response};
use serde::Serialize;
use serde_json::Value;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::proxy_response_handler::AuthResponse;

#[derive(Serialize)]
struct LogData<'a> {
    request: &'a Option<BTreeMap<String, Value>>,
    response: &'a Option<BTreeMap<String, Value>>,
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
    {
        let logs_folder = crate::LOG_DIRECTORY.lock().unwrap().clone();
        let request_path = request.0.uri().path();
        let request_path = request_path.split('/').last().unwrap_or(request_path);
        let file_name = Local::now().format("%Y-%m-%d_%H-%M.%f.json").to_string();
        let dir_path = logs_folder.join(request_path);

        std::fs::create_dir_all(dir_path.clone()).context("could not create log folder")?;
        let file = File::create(dir_path.join(file_name))
            .await
            .context("failed to create log file")?;

        let mut writer = BufWriter::new(file);

        let file_contents = LogData {
            request: &deserialized_request,
            response: &deserialized_response,
        };

        let file_contents =
            serde_json::to_string_pretty(&file_contents).context("could not serialize log data")?;

        writer
            .write_all(file_contents.as_bytes())
            .await
            .context("could not write log contents to file")?;

        writer.flush().await?;
    }

    Ok(())
}

pub async fn log_auth(request: &Request<String>, mut response: AuthResponse) -> Result<()> {
    response.session.token = "***".into();

    println!("AUTH: Authenticated as {:?}", response.user);
    Ok(())
}
