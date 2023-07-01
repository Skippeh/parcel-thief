use std::collections::BTreeMap;

use serde::Serialize;

use crate::readers::{localized_text_resource::Language, LoadContext};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QpidArea {
    pub qpid_id: u32,
    pub names: BTreeMap<Language, String>,
}

pub fn read_qpid_areas(
    load_context: &mut LoadContext,
    qpid_areas: &mut Vec<QpidArea>,
) -> Result<(), anyhow::Error> {
    Ok(())
}
