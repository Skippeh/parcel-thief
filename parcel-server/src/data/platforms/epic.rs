use std::{collections::HashMap, fmt::Display, sync::Arc};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use futures_util::TryFutureExt;
use reqwest::Client;
use serde::{Deserialize, Deserializer};

use crate::data::redis_client::RedisClient;

#[derive(Debug, thiserror::Error)]
pub enum VerifyTokenError {
    UnexpectedApiResponse(anyhow::Error),
    InvalidApiResponse(reqwest::Error),
    InvalidToken,
}

impl Display for VerifyTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyTokenError::UnexpectedApiResponse(err) => write!(
                f,
                "An unexpected response was received from the Epic web api: {}",
                err
            ),
            VerifyTokenError::InvalidApiResponse(err) => write!(
                f,
                "Invalid response received from the Epic web api: {}",
                err
            ),
            VerifyTokenError::InvalidToken => write!(f, "The user token could not be verified"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserEpicId {
    pub account_id: String,
    pub client_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_id: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", untagged)]
pub enum VerifyTokenResponse {
    Active {
        active: bool,
        scope: String,
        token_type: String,
        expires_in: i64,
        #[serde(deserialize_with = "date_from_string")]
        expires_at: DateTime<Utc>,
        account_id: String,
        client_id: String,
        application_id: String,
    },
    NotActive {
        active: bool,
    },
}

pub struct Epic {
    redis_client: Arc<RedisClient>,
    redis_prefix: String,
    web_client: Client,
}

const APP_ID: &str = "fghi4567yfjXGhAtozprq2mNWIovtrZG";
const TOKEN_INFO_URL: &str = "https://api.epicgames.dev/epic/oauth/v1/tokenInfo";
const ACCOUNTS_INFO_URL: &str = "https://api.epicgames.dev/epic/id/v1/accounts";

impl Epic {
    pub fn new(redis_client: Arc<RedisClient>, redis_prefix: &str) -> Result<Self, reqwest::Error> {
        let web_client = Client::builder().user_agent("DS").build()?;
        Ok(Self {
            redis_client,
            redis_prefix: redis_prefix.to_owned(),
            web_client,
        })
    }

    pub async fn verify_token(&self, token: &str) -> Result<UserEpicId, VerifyTokenError> {
        let response = self
            .web_client
            .post(TOKEN_INFO_URL)
            .form(&[("token", token)])
            .send()
            .map_err(VerifyTokenError::InvalidApiResponse)
            .await?;

        dbg!(&response);

        match response.json::<VerifyTokenResponse>().await {
            Ok(response) => match response {
                VerifyTokenResponse::NotActive { active: _ } => Err(VerifyTokenError::InvalidToken),
                VerifyTokenResponse::Active {
                    active,
                    scope: _,
                    token_type: _,
                    expires_in: _,
                    expires_at,
                    account_id,
                    client_id,
                    application_id,
                } => {
                    if !active {
                        return Err(VerifyTokenError::InvalidToken);
                    }

                    if application_id != APP_ID {
                        return Err(VerifyTokenError::InvalidToken);
                    }

                    if expires_at <= Utc::now() {
                        return Err(VerifyTokenError::InvalidToken);
                    }

                    Ok(UserEpicId {
                        account_id,
                        client_id,
                    })
                }
            },
            Err(err) => Err(VerifyTokenError::UnexpectedApiResponse(err.into())),
        }
    }

    pub async fn get_account_infos(
        &self,
        token: &str,
        account_ids: &[&str],
    ) -> Result<HashMap<String, AccountInfo>, anyhow::Error> {
        let query_params = account_ids
            .iter()
            .map(|id| ("accountId", *id))
            .collect::<Vec<_>>();

        let response = self
            .web_client
            .get(ACCOUNTS_INFO_URL)
            .bearer_auth(token)
            .query(&query_params)
            .send()
            .await?;

        let account_infos = response.json::<Vec<AccountInfo>>().await?;
        let result = HashMap::from_iter(
            account_infos
                .into_iter()
                .map(|info| (info.account_id.clone(), info)),
        );

        Ok(result)
    }
}

fn date_from_string<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";
    let date_str = String::deserialize(deserializer)?;

    let date = NaiveDateTime::parse_from_str(&date_str, FORMAT)
        .map_err(|_err| serde::de::Error::custom("Invalid date format"))?;

    Ok(Utc.from_utc_datetime(&date))
}
