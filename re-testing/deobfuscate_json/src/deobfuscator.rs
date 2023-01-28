use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt, BufReader, BufWriter},
};

#[derive(Debug, Serialize, Deserialize)]
struct LogFile {
    request: Option<Value>,
    response: Option<Value>,
}

#[async_recursion]
pub async fn deobfuscate_json_logs(
    logs_directory: &Path,
    string_pairs: &BTreeMap<String, Vec<String>>,
    skip_deobfuscated: bool,
) -> Result<()> {
    let files_in_dir = std::fs::read_dir(logs_directory);

    match files_in_dir {
        Err(err) => {
            println!(
                "could not access directory \"{}\": {}",
                logs_directory.display(),
                err
            );
            Ok(())
        }
        Ok(files_in_dir) => {
            for item in files_in_dir {
                let item = item?;
                let path = item.path();

                if path.is_dir() {
                    deobfuscate_json_logs(&path, string_pairs, skip_deobfuscated).await?;
                } else if path.is_file() {
                    let extension = path.extension();

                    match extension {
                        None => continue,
                        Some(extension) => {
                            // ignore non json files
                            if !extension.eq_ignore_ascii_case("json") {
                                continue;
                            }

                            // Skip already deobfuscated files
                            if path.to_string_lossy().ends_with("_d.json") {
                                continue;
                            }

                            let mut deobfuscated_path = path.to_owned();
                            let mut file_name = deobfuscated_path
                                .with_extension("")
                                .file_name()
                                .unwrap()
                                .to_string_lossy()
                                .to_string();
                            file_name.push_str("_d.json");
                            deobfuscated_path.set_file_name(file_name);

                            // Skip this file in case there's already a deobfuscated file of it
                            if skip_deobfuscated && deobfuscated_path.exists() {
                                continue;
                            }

                            deobfuscate_log(
                                path.as_path(),
                                deobfuscated_path.as_path(),
                                string_pairs,
                            )
                            .await?;
                        }
                    }
                }
            }

            Ok(())
        }
    }
}

async fn deobfuscate_log(
    input_path: &Path,
    output_path: &Path,
    string_pairs: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    let mut reader = BufReader::new(File::open(input_path).await?);
    let mut input_contents = String::new();
    reader.read_to_string(&mut input_contents).await?;
    let mut log =
        serde_json::from_str::<LogFile>(&input_contents).context("invalid log file format")?;

    if let Some(request) = log.request {
        log.request = Some(replace_keys(request, string_pairs));
    }

    if let Some(response) = log.response {
        log.response = Some(replace_keys(response, string_pairs));
    }

    let output_json = serde_json::to_string_pretty(&log)?;
    let mut output_file = BufWriter::new(File::create(output_path).await?);
    output_file
        .write_all(output_json.as_bytes())
        .await
        .context("could not write output json to file")?;
    output_file.flush().await?;

    Ok(())
}

fn replace_keys(obj: Value, string_pairs: &BTreeMap<String, Vec<String>>) -> Value {
    match obj {
        Value::Array(array) => {
            let mut new_val = Vec::new();
            for val in array {
                new_val.push(replace_keys(val, string_pairs));
            }
            Value::Array(new_val)
        }
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (key, val) in map.into_iter() {
                let new_key = deobfuscate_key(&key, string_pairs);
                let new_val = match val {
                    Value::Object(_) => replace_keys(val, string_pairs),
                    Value::Array(_) => replace_keys(val, string_pairs),
                    _ => val.clone(),
                };
                new_map.insert(new_key, new_val);
            }
            Value::Object(new_map)
        }
        _ => obj,
    }
}

fn deobfuscate_key(key: &str, string_pairs: &BTreeMap<String, Vec<String>>) -> String {
    match string_pairs.get(key) {
        Some(keys) => {
            let mut new_key = key.to_owned();
            new_key.push_str(": ");

            for (i, key) in keys.iter().enumerate() {
                new_key.push_str(key);

                if i < keys.len() - 1 {
                    new_key.push_str(" / ");
                }
            }

            new_key
        }
        None => key.to_owned(),
    }
}
