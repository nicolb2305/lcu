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

/// Creates a custom game with tournament draft on Summoner's Rift.
///
/// # Errors
/// Fails if client api cannot be reached.
pub async fn create_custom(client: &Client) -> Result<(), Error> {
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
