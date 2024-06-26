use std::num::ParseIntError;
use thiserror::Error;
use types::ApiError;

#[cfg(feature = "actions")]
pub mod actions;
#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "ddragon")]
pub mod ddragon;
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
    #[error("No queue found for current lobby")]
    QueueNotFoundError,
    #[error("Wrong gamemode")]
    WrongGameMode,
    #[error("Could not move player to given location")]
    PlayerMove,
    #[error("Custom error")]
    Custom(String),
}

#[cfg(test)]
mod tests {
    use crate::{actions::select_champion, client::Client, types::LolLobbySubteamDataDto, Error};

    #[tokio::test]
    async fn get_lobby() -> Result<(), Error> {
        let client = Client::new()?;
        let lobby = client.get_lol_lobby_v2_lobby().await?;
        dbg!(lobby);
        Ok(())
    }

    #[tokio::test]
    async fn put_subteam() -> Result<(), Error> {
        let client = Client::new()?;
        client
            .put_lol_lobby_v2_lobby_subteam_data(&LolLobbySubteamDataDto {
                subteam_index: 1,
                intra_subteam_position: 2,
            })
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn get_mastery() -> Result<(), Error> {
        let client = Client::new()?;
        let mastery = client
            .get_lol_champion_mastery_v1_local_player_champion_mastery()
            .await?;
        dbg!(mastery);
        Ok(())
    }

    #[tokio::test]
    async fn get_challenges() -> Result<(), Error> {
        let client = Client::new()?;
        let challenges = client
            .get_lol_challenges_v1_challenges_local_player()
            .await?;
        dbg!(challenges);
        Ok(())
    }

    #[tokio::test]
    async fn pick_champ() -> Result<(), Error> {
        let client = Client::new()?;
        select_champion(&client, 4).await.unwrap();
        Ok(())
    }
}
