use crate::{
    client::Client,
    types::{
        LolChatConversationMessageResource, LolChatFriendResource, LolLobbyLobbyChangeGameDto,
        LolLobbyLobbyCustomGameConfiguration, LolLobbyLobbyCustomGameLobby,
        LolLobbyLobbyInvitationDto, LolLobbyQueueCustomGameSpectatorPolicy,
        LolLobbyQueueGameTypeConfig,
    },
    Error,
};
use futures::future::try_join_all;
use itertools::Itertools;
use rand::prelude::*;

/// Gets all players in the current lobby, generates two random teams and posts them
/// in the lobby chat.
///
/// # Errors
/// Fails if the custom game chat cannot be found or the client api cannot be reached.
pub async fn randomize_teams(client: &Client) -> Result<(), Error> {
    // Create teams
    let lobby = client.get_lol_lobby_v2_lobby().await?;
    let mut players: Vec<&str> = lobby
        .members
        .iter()
        .map(|p| p.summoner_name.as_ref())
        .collect();
    players.shuffle(&mut thread_rng());
    let (team1, team2) = players
        .chunks((players.len() / 2) + (players.len() % 2))
        .map(|x| x.join("\n"))
        .collect_tuple()
        .ok_or(Error::TeamCreation)?;
    let teams_output = format!(".\nTeam 1:\n{team1}\n----------\nTeam 2:\n{team2}");

    // Find custom game chat
    let conversations = client.get_lol_chat_v1_conversations().await?;

    let custom_game_chat = conversations
        .iter()
        .find(|x| x.type_ == "customGame")
        .ok_or(Error::LobbyNotFound)?;

    // Post teams in chat
    let post_body = LolChatConversationMessageResource {
        body: teams_output,
        type_: "groupchat".to_string(),
        ..Default::default()
    };
    client
        .post_lol_chat_v1_conversations_by_id_messages(&custom_game_chat.id, post_body)
        .await?;

    Ok(())
}

pub enum DraftType {
    BlindPick = 1,
    Draft = 2,
    AllRandom = 4,
    TorunamentDraft = 6,
}

pub enum Map {
    SummonersRift = 11,
    HowlingAbyss = 12,
}

/// Creates a custom game with tournament draft on Summoner's Rift.
///
/// # Errors
/// Fails if client api cannot be reached.
pub async fn create_custom(client: &Client, draft_type: DraftType, map: Map) -> Result<(), Error> {
    let queue_config = LolLobbyQueueGameTypeConfig {
        id: draft_type as i64,
        ..Default::default()
    };

    let game_mode = match map {
        Map::SummonersRift => "CLASSIC",
        Map::HowlingAbyss => "ARAM",
    };

    let config = LolLobbyLobbyCustomGameConfiguration {
        map_id: map as i32,
        game_mode: game_mode.to_string(),
        mutators: queue_config.clone(),
        game_type_config: queue_config,
        spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy::AllAllowed,
        team_size: 5,
        max_player_count: 10,
        ..Default::default()
    };

    let custom_game_lobby = LolLobbyLobbyCustomGameLobby {
        lobby_name: "Gretta".to_string(),
        lobby_password: "test".to_string(),
        configuration: config,
        ..Default::default()
    };

    let lobby = LolLobbyLobbyChangeGameDto {
        // Custom game
        queue_id: 0,
        is_custom: true,
        custom_game_lobby: Some(custom_game_lobby),
        ..Default::default()
    };

    client.post_lol_lobby_v2_lobby(lobby).await?;

    Ok(())
}

/// Gets every summoner on the friends list logged into league of legends who are online
/// or away.
///
/// # Errors
/// Fails if client api cannot be reached.
pub async fn get_online_friends(client: &Client) -> Result<Vec<LolChatFriendResource>, Error> {
    Ok(client
        .get_lol_chat_v1_friends()
        .await?
        .into_iter()
        .filter(|f| {
            f.product == "league_of_legends"
                && (f.availability == "chat" || f.availability == "away")
        })
        .collect())
}

/// Invites summoners with given ids to the current lobby.
///
/// # Errors
/// Fails if client api cannot be reached, or if player is not in a lobby.
pub async fn invite_to_lobby(client: &Client, summoners: &[u64]) -> Result<(), Error> {
    let body = summoners
        .iter()
        .map(|id| LolLobbyLobbyInvitationDto {
            to_summoner_id: *id,
            ..Default::default()
        })
        .collect();
    client.post_lol_lobby_v2_lobby_invitations(body).await?;
    Ok(())
}

/// Fetches the last 200 games from the player's match history and sends any custom
/// games with 10 participants to the pasanapi.
///
/// # Errors
/// Fails if the client api cannot be reached, or if the pasanapi cannot be reached.
pub async fn post_custom_games_to_pasanapi(
    client: &Client,
    num_matches_to_check: u8,
) -> Result<(), Error> {
    let req_client = reqwest::Client::new();

    let match_history = client
        .get_lol_match_history_v1_products_lol_current_summoner_matches(
            None,
            Some(num_matches_to_check),
        )
        .await?
        .games
        .games;

    let match_history = match_history
        .into_iter()
        .filter(|x| x.map_id == 11 && x.game_type == "CUSTOM_GAME" && x.game_mode == "CLASSIC")
        .map(|x| client.get_lol_match_history_v1_games_by_game_id(x.game_id));

    let post_responses = try_join_all(match_history)
        .await?
        .into_iter()
        .filter(|x| x.participants.len() == 10)
        .map(|x| {
            req_client
                .post("https://api.påsan.com/match")
                .json(&x)
                .send()
        });

    let responses = try_join_all(post_responses).await?;

    let num_inserted = responses
        .into_iter()
        .filter(|x| x.status().is_success())
        .count();

    log::info!("Successfully sent {num_inserted} custom games");

    Ok(())
}

/// Checks players previous 10 games played and invites everyone from the first custom
/// game found.
///
/// # Errors
/// Fails if client api cannot be reached, or if no recent custom games can be found.
pub async fn invite_from_previous(client: &Client) -> Result<(), Error> {
    let match_history = client
        .get_lol_match_history_v1_products_lol_current_summoner_matches(None, Some(10))
        .await?
        .games
        .games;

    let last_game = match_history
        .into_iter()
        .find(|x| x.game_type == "CUSTOM_GAME")
        .ok_or(Error::NoGamesInMatchHistory)?;

    let summoners: Vec<_> = client
        .get_lol_match_history_v1_games_by_game_id(last_game.game_id)
        .await?
        .participant_identities
        .into_iter()
        .map(|x| x.player.summoner_id)
        .collect();

    invite_to_lobby(client, &summoners).await?;

    Ok(())
}
