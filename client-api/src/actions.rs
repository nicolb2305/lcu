use crate::{
    client::Client,
    types::{
        LolChatConversationMessageResource, LolChatFriendResource, LolLobbyInvitationType,
        LolLobbyLobbyChangeGameDto, LolLobbyLobbyCustomGameConfiguration,
        LolLobbyLobbyCustomGameLobby, LolLobbyLobbyInvitationDto,
        LolLobbyQueueCustomGameSpectatorPolicy, LolLobbyQueueGameTypeConfig,
    },
};
use eyre::{ContextCompat, Result};
use itertools::Itertools;
use rand::prelude::*;

/// Gets all players in the current lobby, generates two random teams and posts them
/// in the lobby chat.
///
/// # Errors
/// Fails if the custom game chat cannot be found or the client api cannot be reached.
pub fn randomize_teams(client: &Client) -> Result<()> {
    // Create teams
    let lobby = client.get_lol_lobby_v2_lobby()?;
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
        .context("Failed to create two teams")?;
    let teams_output = format!(".\nTeam 1:\n{team1}\n----------\nTeam 2:\n{team2}");

    // Find custom game chat
    let conversations = client.get_lol_chat_v1_conversations()?;

    let custom_game_chat = conversations
        .iter()
        .find(|x| x.type_ == "customGame")
        .context("Failed to find custom game chat")?;

    // Post teams in chat
    let post_body = LolChatConversationMessageResource {
        body: teams_output,
        type_: "groupchat".to_string(),
        ..Default::default()
    };
    client.post_lol_chat_v1_conversations_by_id_messages(&custom_game_chat.id, post_body)?;

    Ok(())
}

/// Creates a custom game with tournament draft on Summoner's Rift.
///
/// # Errors
/// Fails if client api cannot be reached.
pub fn create_custom(client: &Client) -> Result<()> {
    let queue_config = LolLobbyQueueGameTypeConfig {
        // Tournament draft
        id: 6,
        ..Default::default()
    };

    let config = LolLobbyLobbyCustomGameConfiguration {
        // Summoner's Rift
        map_id: 11,
        game_mode: "CLASSIC".to_string(),
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

    client.post_lol_lobby_v2_lobby(lobby)?;

    Ok(())
}

pub fn get_online_friends(client: &Client) -> Result<Vec<LolChatFriendResource>> {
    Ok(client
        .get_lol_chat_v1_friends()?
        .into_iter()
        .filter(|f| {
            f.product == "league_of_legends"
                && (f.availability == "chat" || f.availability == "away")
        })
        .collect())
}

pub fn invite_to_lobby(client: &Client, summoners: &[u64]) -> Result<()> {
    let body = summoners
        .iter()
        .map(|id| LolLobbyLobbyInvitationDto {
            invitation_type: LolLobbyInvitationType::Lobby,
            to_summoner_id: *id,
            // to_summoner_name: name.clone(),
            ..Default::default()
        })
        .collect();
    client.post_lol_lobby_v2_lobby_invitations(body)?;
    Ok(())
}
