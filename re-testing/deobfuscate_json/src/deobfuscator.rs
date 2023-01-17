use std::{collections::BTreeMap, path::Path};

use anyhow::Result;

pub async fn deobfuscate_json_logs(
    logs_directory: &Path,
    output_directory: &Path,
    string_pairs: &BTreeMap<String, Vec<String>>,
) -> Result<()> {
    anyhow::bail!("unimplemented");
}
