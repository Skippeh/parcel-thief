use std::{collections::BTreeMap, path::Path};

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, BufReader};

pub async fn parse_string_pairs(file_path: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let file = tokio::fs::File::open(file_path)
        .await
        .context("could not open file")?;
    let mut reader = BufReader::new(file);
    let mut file_contents = String::with_capacity(100000);
    reader
        .read_to_string(&mut file_contents)
        .await
        .context("failed to read contents of file")?;

    file_contents = file_contents.replace('\r', "");
    let mut lines = file_contents.split('\n');

    while let Some(mut strings) = read_until_double_newline(&mut lines) {
        if strings.len() == 2 {
            strings.reverse();

            let map = result.get_mut(strings[0]);

            match map {
                Some(vec) => {
                    let string = strings[1].into();
                    if !vec.contains(&string) {
                        vec.push(string)
                    }
                }
                None => {
                    let vec = vec![strings[1].into()];
                    result.insert(strings[0].into(), vec);
                }
            }
        }
    }

    assert_eq!(lines.next(), None);

    // Sort values
    for (_, val) in result.iter_mut() {
        val.sort_unstable();
    }

    Ok(result)
}

fn read_until_double_newline<'a>(lines: &mut std::str::Split<'a, char>) -> Option<Vec<&'a str>> {
    let mut result = Vec::<&'a str>::new();
    let mut is_some = false;

    for line in lines.by_ref() {
        is_some = true;

        if line.is_empty() {
            break;
        }

        // separate text from its address
        let line = line.split_once(',').map(|d| d.0);

        if let Some(line) = line {
            result.push(line)
        }
    }

    if !result.is_empty() || is_some {
        Some(result)
    } else {
        None
    }
}
