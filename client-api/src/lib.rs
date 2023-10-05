use reqwest::header::InvalidHeaderValue;
use std::num::ParseIntError;
use thiserror::Error;
use types::ApiError;

pub mod actions;
pub mod client;
pub mod endpoints;
pub mod types;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Api request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Api returned error: {0}")]
    ApiError(ApiError),
    #[error("Deserialization of api response failed: {0}")]
    ApiErrorDeserialization(#[from] serde_json::Error),
    #[error("Regex construction failed: {0}")]
    RegexContruction(#[from] regex_lite::Error),
    #[error("Client could not be found")]
    ClientNotFound,
    #[error("Client port argument could not be found")]
    PortNotFound,
    #[error("Client auth argument could not be found")]
    AuthNotFound,
    #[error("Parsing of port number failed: {0}")]
    PortParsing(#[from] ParseIntError),
    #[error("Auth header construction failed: {0}")]
    InvalidHeader(#[from] InvalidHeaderValue),
    #[error("Team creation failed")]
    TeamCreation,
    #[error("Player is not in a lobby")]
    LobbyNotFound,
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, Error};

    #[tokio::test]
    async fn get_lobby() -> Result<(), Error> {
        let client = Client::new()?;
        let lobby = client.get_lol_lobby_v2_lobby().await?;
        dbg!(lobby);
        Ok(())
    }
}
