use std::{collections::HashMap, fmt::Display};

use anyhow::Context;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Deserialize;

const APP_ID: u32 = 1850570;
const URL_AUTH_USER_TICKET: &str =
    "https://api.steampowered.com/ISteamUserAuth/AuthenticateUserTicket/v1/";
const URL_GET_PLAYER_SUMMARIES: &str =
    "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/";

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    response: Response<T>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Response<T> {
    Ok(T),
    Error { error: ErrorResponse },
}

#[derive(Debug, Deserialize)]
struct ParamsResponse<T> {
    params: T,
}

#[derive(Debug, Deserialize)]
struct PlayersResponse {
    players: Vec<ReqPlayerSummary>,
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    #[serde(rename = "errorcode")]
    code: i32,
    #[serde(rename = "errordesc")]
    description: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AuthenticateUserTicketParams {
    result: String,
    #[serde(rename = "steamid")]
    steam_id: String,
    #[serde(rename = "ownersteamid")]
    owner_steam_id: String,
    #[serde(rename = "vacbanned")]
    vac_banned: bool,
    #[serde(rename = "publisherbanned")]
    publisher_banned: bool,
}

#[derive(Debug)]
pub struct UserSteamId {
    pub steam_id: u64,
    pub owner_steam_id: u64,
}

impl Display for UserSteamId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(SteamId: {}, OwnerId: {})",
            &self.steam_id, &self.owner_steam_id
        )
    }
}

impl UserSteamId {
    pub fn new(steam_id: u64, owner_steam_id: u64) -> Self {
        Self {
            steam_id,
            owner_steam_id,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ReqPlayerSummary {
    #[serde(rename = "steamid")]
    steam_id: String,
    #[serde(rename = "personaname")]
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct PlayerSummary {
    pub steam_id: u64,
    pub name: String,
}

impl PlayerSummary {
    pub fn new(steam_id: u64, name: String) -> Self {
        Self { steam_id, name }
    }
}

#[derive(Debug)]
pub struct Steam {
    api_key: String,
    web_client: Client,
}

impl Steam {
    pub fn new(api_key: String) -> Result<Self, reqwest::Error> {
        let client = Client::builder().user_agent("DS").build()?;
        Ok(Self {
            api_key,
            web_client: client,
        })
    }

    /// Verifies the user auth ticket and if successful returns the user steam id and owner id (owner id is different if the game is family shared)
    pub async fn verify_user_auth_ticket(
        &self,
        ticket: &[u8],
    ) -> Result<UserSteamId, anyhow::Error> {
        let ticket_str: String = hex::encode(ticket);

        let response = self
            .create_request(reqwest::Method::GET, URL_AUTH_USER_TICKET)?
            .query(&[("appid", APP_ID)])
            .query(&[("ticket", &ticket_str)])
            .send()
            .await?;

        match response.status() {
            StatusCode::OK => {
                let response = response
                    .json::<ApiResponse<ParamsResponse<AuthenticateUserTicketParams>>>()
                    .await
                    .context("Unexpected api response from steam api")?;

                match response.response {
                    Response::Ok(data) => {
                        let params = data.params;
                        let steam_id = params.steam_id.parse::<u64>()?;
                        let owner_steam_id = params.owner_steam_id.parse::<u64>()?;
                        Ok(UserSteamId::new(steam_id, owner_steam_id))
                    }
                    Response::Error { error } => {
                        anyhow::bail!(
                            "Steam response was not successful: {} (error code {})",
                            error.description,
                            error.code
                        );
                    }
                }
            }
            default => anyhow::bail!("Unexpected response: {}", default),
        }
    }

    pub async fn get_player_summaries(
        &self,
        user_ids: Vec<u64>,
    ) -> Result<HashMap<u64, PlayerSummary>, anyhow::Error> {
        let mut builder = self.create_request(reqwest::Method::GET, URL_GET_PLAYER_SUMMARIES)?;

        for user_id in user_ids {
            builder = builder.query(&[("steamids", user_id.to_string())]);
        }

        let response = builder.send().await?;

        match response.status() {
            StatusCode::OK => {
                let response = response
                    .json::<ApiResponse<PlayersResponse>>()
                    .await
                    .context("Unexpected api response from steam api")?;

                match response.response {
                    Response::Ok(data) => {
                        let mut hashmap = HashMap::<u64, PlayerSummary>::new();

                        for player in data.players {
                            let steam_id = player.steam_id.parse::<u64>()?;

                            hashmap.insert(steam_id, PlayerSummary::new(steam_id, player.name));
                        }

                        Ok(hashmap)
                    }
                    Response::Error { error } => {
                        anyhow::bail!(
                            "Steam response was not successful: {} (error code {})",
                            error.description,
                            error.code
                        );
                    }
                }
            }
            default => anyhow::bail!("Unexpected response: {}", default),
        }
    }

    fn create_request(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> Result<RequestBuilder, anyhow::Error> {
        Ok(self
            .web_client
            .request(method, url)
            .query(&[("key", &self.api_key)]))
    }
}
