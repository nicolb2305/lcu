use std::num::ParseIntError;
use thiserror::Error;
use types::ApiError;

#[cfg(feature = "actions")]
pub mod actions;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "endpoints")]
pub mod endpoints;
#[cfg(feature = "types")]
pub mod types;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "client")]
    #[error("Api request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Api returned error: {0}")]
    ApiError(ApiError),
    #[error("Deserialization of api response failed: {0}")]
    ApiErrorDeserialization(#[from] serde_json::Error),
    #[error("Client could not be found")]
    ClientNotFound,
    #[error("Client port argument could not be found")]
    PortNotFound,
    #[error("Client auth argument could not be found")]
    AuthNotFound,
    #[error("Parsing of port number failed: {0}")]
    PortParsing(#[from] ParseIntError),
    #[error("Invalid port: {0}")]
    InvalidPort(u16),
    #[error("Failed to parse base url: {0}")]
    BaseUrlConstruction(#[from] url::ParseError),
    #[cfg(feature = "client")]
    #[error("Auth header construction failed: {0}")]
    InvalidHeader(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Team creation failed")]
    TeamCreation,
    #[error("Player is not in a lobby")]
    LobbyNotFound,
    #[error("No games in match history")]
    NoGamesInMatchHistory,
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
