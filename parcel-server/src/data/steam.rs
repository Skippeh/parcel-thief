use std::{collections::HashMap, fmt::Display, sync::Arc};

use anyhow::Context;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Deserialize;

use super::redis_client::RedisClient;

const APP_ID: u32 = 1850570;
const URL_AUTH_USER_TICKET: &str =
    "https://api.steampowered.com/ISteamUserAuth/AuthenticateUserTicket/v1/";
const URL_GET_PLAYER_SUMMARIES: &str =
    "https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/";

pub type SteamId = i64;

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

#[derive(Debug, Deserialize, thiserror::Error)]
struct ErrorResponse {
    #[serde(rename = "errorcode")]
    code: i32,
    #[serde(rename = "errordesc")]
    description: String,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Steam response was not successful: {} (error code {})",
            self.description, self.code
        )
    }
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
    pub steam_id: SteamId,
    pub owner_steam_id: SteamId,
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
    pub fn new(steam_id: SteamId, owner_steam_id: SteamId) -> Self {
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
    pub steam_id: SteamId,
    pub name: String,
}

impl PlayerSummary {
    pub fn new(steam_id: SteamId, name: String) -> Self {
        Self { steam_id, name }
    }
}

#[derive(Debug)]
pub struct Steam {
    api_key: String,
    web_client: Client,
    redis_client: Arc<RedisClient>,
    redis_prefix: String,
}

pub trait AsHexString {
    fn as_hex_string(&self) -> String;
}

impl AsHexString for String {
    fn as_hex_string(&self) -> String {
        self.clone()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum VerifyUserAuthTicketError {
    UnexpectedApiResponse(anyhow::Error),
    InvalidApiResponse(reqwest::Error),
    InvalidTicket,
}

impl Display for VerifyUserAuthTicketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyUserAuthTicketError::UnexpectedApiResponse(err) => write!(
                f,
                "An unexpected response was received from the Steam web api: {}",
                err
            ),
            VerifyUserAuthTicketError::InvalidApiResponse(err) => {
                write!(
                    f,
                    "Invalid response received from the Steam web api: {}",
                    err
                )
            }
            VerifyUserAuthTicketError::InvalidTicket => {
                write!(f, "The user ticket could not be verified")
            }
        }
    }
}

impl Steam {
    pub fn new(
        api_key: String,
        redis_client: Arc<RedisClient>,
        redis_prefix: &str,
    ) -> Result<Self, reqwest::Error> {
        let web_client = Client::builder().user_agent("DS").build()?;
        Ok(Self {
            api_key,
            web_client,
            redis_client,
            redis_prefix: redis_prefix.into(),
        })
    }

    /// Verifies the user auth ticket and if successful returns the user steam id and owner id (owner id is different if the game is family shared)
    pub async fn verify_user_auth_ticket<T>(
        &self,
        ticket: &T,
    ) -> Result<UserSteamId, VerifyUserAuthTicketError>
    where
        T: AsHexString,
    {
        let ticket_str: String = ticket.as_hex_string();

        let response = self
            .create_request(reqwest::Method::GET, URL_AUTH_USER_TICKET)
            .query(&[("appid", APP_ID)])
            .query(&[("ticket", &ticket_str)])
            .send()
            .await
            .map_err(VerifyUserAuthTicketError::InvalidApiResponse)?;

        match response.status() {
            StatusCode::OK => {
                let response = response
                    .json::<ApiResponse<ParamsResponse<AuthenticateUserTicketParams>>>()
                    .await
                    .map_err(|err| VerifyUserAuthTicketError::UnexpectedApiResponse(err.into()))?;

                match response.response {
                    Response::Ok(data) => {
                        let params = data.params;
                        let steam_id = params.steam_id.parse::<SteamId>().map_err(|err| {
                            VerifyUserAuthTicketError::UnexpectedApiResponse(err.into())
                        })?;
                        let owner_steam_id =
                            params.owner_steam_id.parse::<SteamId>().map_err(|err| {
                                VerifyUserAuthTicketError::UnexpectedApiResponse(err.into())
                            })?;
                        Ok(UserSteamId::new(steam_id, owner_steam_id))
                    }
                    Response::Error { error } => Err(match error.code {
                        // 3 = invalid ticket format
                        // 101 = invalid ticket (already used or expired for example)
                        3 | 101 => VerifyUserAuthTicketError::InvalidTicket,
                        _ => VerifyUserAuthTicketError::UnexpectedApiResponse(error.into()),
                    }),
                }
            }
            default => Err(VerifyUserAuthTicketError::UnexpectedApiResponse(
                anyhow::anyhow!("Unexpected response status: {}", default),
            )),
        }
    }

    pub async fn get_player_summaries(
        &self,
        user_ids: &[&SteamId],
    ) -> Result<HashMap<SteamId, PlayerSummary>, anyhow::Error> {
        let mut builder = self.create_request(reqwest::Method::GET, URL_GET_PLAYER_SUMMARIES);

        builder = builder.query(
            &user_ids
                .iter()
                .map(|id| ("steamids", id.to_string()))
                .collect::<Vec<_>>(),
        );

        let response = builder.send().await?;

        match response.status() {
            StatusCode::OK => {
                let response = response
                    .json::<ApiResponse<PlayersResponse>>()
                    .await
                    .context("Unexpected api response from steam api")?;

                match response.response {
                    Response::Ok(data) => {
                        let mut hashmap = HashMap::<SteamId, PlayerSummary>::new();

                        for player in data.players {
                            let steam_id = player.steam_id.parse::<SteamId>()?;

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

    fn create_request(&self, method: reqwest::Method, url: &str) -> RequestBuilder {
        self.web_client
            .request(method, url)
            .query(&[("key", &self.api_key)])
    }
}
