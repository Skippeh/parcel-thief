use std::path::Path;

use parcel_common::api_types::frontend::settings::WhitelistEntry;

#[derive(Debug, Clone, Default)]
pub struct Whitelist(Vec<WhitelistEntry>);

impl Whitelist {
    pub fn into_inner(self) -> Vec<WhitelistEntry> {
        self.0
    }
}

impl std::ops::Deref for Whitelist {
    type Target = Vec<WhitelistEntry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Whitelist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct WhitelistPersist;

#[async_trait::async_trait]
impl super::settings::Persist<Whitelist> for WhitelistPersist {
    // The file format looks like the following:
    // There is one line for each entry:
    // provider_id;name_reference
    //
    // Name reference can include ';', only the first one is used to separate the two parts.
    // If there is no ';' then there is no name reference specified for the provider_id.
    // The final line can be a blank line. Otherwise it must follow the format above.
    //
    // On steam, the provider id is the steamid64. On epic it's the account id.

    async fn read_file(file_path: &Path) -> Result<Whitelist, anyhow::Error> {
        let contents = tokio::fs::read_to_string(file_path).await?;
        let mut entries = Vec::new();

        for line in contents.lines() {
            let parts = line.split_once(';');

            if let Some((provider_id, name_reference)) = parts {
                entries.push(WhitelistEntry {
                    provider_id: provider_id.to_string(),
                    name_reference: Some(name_reference.to_string()),
                })
            } else {
                entries.push(WhitelistEntry {
                    provider_id: line.to_string(),
                    name_reference: None,
                })
            }
        }

        Ok(Whitelist(entries))
    }

    async fn write_file(file_path: &Path, data: &Whitelist) -> Result<(), anyhow::Error> {
        let mut contents = String::with_capacity(1024);

        for entry in data.0.iter() {
            contents.push_str(&format!("{}", entry.provider_id.replace('\n', " ")));

            if let Some(name_reference) = &entry.name_reference {
                contents.push_str(&format!(";{}", name_reference.replace('\n', " ")));
            }

            contents.push('\n');
        }

        tokio::fs::write(file_path, contents.as_bytes()).await?;
        Ok(())
    }
}
