use crate::{
    client::Client,
    types::{
        LolChatConversationMessageResource, LolChatFriendResource, LolLobbyLobbyChangeGameDto,
        LolLobbyLobbyCustomGameConfiguration, LolLobbyLobbyCustomGameLobby,
        LolLobbyLobbyInvitationDto, LolLobbyLobbyParticipantDto,
        LolLobbyQueueCustomGameSpectatorPolicy, LolLobbyQueueGameTypeConfig,
        LolLobbySubteamDataDto,
    },
    Error,
};
use futures::future::try_join_all;
use itertools::Itertools;
use rand::prelude::*;

enum Queues {
    Arena,
    Arena16,
    Other,
}

impl From<i32> for Queues {
    fn from(value: i32) -> Self {
        match value {
            1700 => Self::Arena,
            1710 => Self::Arena16,
            _ => Self::Other,
        }
    }
}

/// Gets all players in the current lobby, generates two random teams and posts them
/// in the lobby chat.
///
/// # Errors
/// Fails if the custom game chat cannot be found or the client api cannot be reached.
pub async fn randomize_teams(client: &Client) -> Result<(), Error> {
    // Create teams
    let lobby = client.get_lol_lobby_v2_lobby().await?;

    dbg!(&lobby);

    let gamemode: Queues = client
        .get_lol_lobby_v1_parties_gamemode()
        .await?
        .queue_id
        .ok_or(Error::QueueNotFoundError)?
        .into();

    let mut players: Vec<_> = lobby.members.iter().collect();

    dbg!(&players);

    players.shuffle(&mut thread_rng());

    dbg!(&players);

    // Intentionally ignores all future queues
    #[allow(clippy::match_wildcard_for_single_variants)]
    let team_size = match gamemode {
        Queues::Arena | Queues::Arena16 => 2,
        _ => players.len() / 2,
    };

    let teams = players.chunks(team_size).collect_vec();

    #[allow(unstable_name_collisions)]
    let teams_output: String = std::iter::once(".\n".to_owned())
        .chain(
            players
                .iter()
                .map(|player| player.summoner_name.as_ref())
                .collect_vec()
                .chunks(team_size)
                .enumerate()
                .map(|(i, team)| format!("Team {}:\n{}", i + 1, team.join("\n")))
                .intersperse("\n----------\n".into()),
        )
        .collect();

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

    // Move players if gamemode is arena
    if matches!(gamemode, Queues::Arena | Queues::Arena16) {
        move_team_members_arena(client, &lobby.local_member, &teams).await?;
    }

    Ok(())
}

struct ArenaTeam<'a> {
    client: &'a Client,
    local_player: usize,
    players: [Option<LolLobbySubteamDataDto>; 16],
}

impl<'a> ArenaTeam<'a> {
    fn from_player_list(
        client: &'a Client,
        local_member: &LolLobbyLobbyParticipantDto,
        teams: &[&[&LolLobbyLobbyParticipantDto]],
    ) -> Self {
        // Create array of current teams and current position of local player
        let mut current_teams = [None; 16];
        let mut current_local_member = None;
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        for (subteam_index, team) in teams.iter().enumerate() {
            for (intra_subteam_position, player) in team.iter().enumerate() {
                if *player == local_member {
                    current_local_member = Some(Self::pos_to_index(LolLobbySubteamDataDto {
                        subteam_index: player.subteam_index.unwrap(),
                        intra_subteam_position: player.intra_subteam_position.unwrap(),
                    }));
                }
                current_teams[Self::pos_to_index(LolLobbySubteamDataDto {
                    subteam_index: player.subteam_index.unwrap(),
                    intra_subteam_position: player.intra_subteam_position.unwrap(),
                })] = Some(LolLobbySubteamDataDto {
                    subteam_index: subteam_index + 1,
                    intra_subteam_position: intra_subteam_position + 1,
                });
            }
        }
        Self {
            client,
            local_player: current_local_member.unwrap(),
            players: current_teams,
        }
    }

    const fn pos_to_index(pos: LolLobbySubteamDataDto) -> usize {
        (pos.subteam_index - 1) * 2 + (pos.intra_subteam_position - 1)
    }

    const fn index_to_pos(idx: usize) -> LolLobbySubteamDataDto {
        LolLobbySubteamDataDto {
            subteam_index: (idx / 2) + 1,
            intra_subteam_position: (idx % 2) + 1,
        }
    }

    fn get_first_player_in_incorrect_position(&self) -> Option<LolLobbySubteamDataDto> {
        self.players.iter().enumerate().find_map(|(idx, player)| {
            player
                .map(|pos| idx != Self::pos_to_index(pos))
                .unwrap_or(false)
                .then(|| Self::index_to_pos(idx))
        })
    }

    fn all_positions_correct(&self) -> bool {
        self.players.iter().enumerate().all(|(idx, player)| {
            player
                .map(|pos| idx == Self::pos_to_index(pos))
                .unwrap_or(true)
        })
    }

    const fn local_player_pos(&self) -> LolLobbySubteamDataDto {
        Self::index_to_pos(self.local_player)
    }

    fn local_player_correct_pos(&self) -> bool {
        self.players[self.local_player] == Some(Self::index_to_pos(self.local_player))
    }

    // fn get_final_position_for_player_at_given_position(
    //     &self,
    //     pos: LolLobbySubteamDataDto,
    // ) -> Option<LolLobbySubteamDataDto> {
    //     self.players[Self::pos_to_index(pos)]
    // }

    fn get_current_position_for_player_with_given_final_position(
        &self,
        pos: LolLobbySubteamDataDto,
    ) -> Option<LolLobbySubteamDataDto> {
        for (idx, player) in self.players.iter().enumerate() {
            if let Some(player) = player {
                if *player == pos {
                    return Some(Self::index_to_pos(idx));
                }
            }
        }
        None
    }

    async fn swap_local_to_pos(&mut self, pos: LolLobbySubteamDataDto) -> Result<(), Error> {
        log::info!("Swapping local player to {pos:?}");
        self.client
            .put_lol_lobby_v2_lobby_subteam_data(&pos)
            .await?;
        let new_local_pos = Self::pos_to_index(pos);
        self.players.swap(self.local_player, new_local_pos);
        self.local_player = new_local_pos;
        Ok(())
    }
}

async fn move_team_members_arena(
    client: &Client,
    local_member: &LolLobbyLobbyParticipantDto,
    teams: &[&[&LolLobbyLobbyParticipantDto]],
) -> Result<(), Error> {
    let mut arena_teams = ArenaTeam::from_player_list(client, local_member, teams);
    log::info!(
        "Initial move of local player to {:?}",
        ArenaTeam::index_to_pos(0)
    );
    arena_teams
        .swap_local_to_pos(ArenaTeam::index_to_pos(0))
        .await?;
    while !arena_teams.all_positions_correct() {
        log::info!("At least one player in incorrect position");
        if arena_teams.local_player_correct_pos() {
            let next_pos = arena_teams
                .get_first_player_in_incorrect_position()
                .ok_or(Error::PlayerMove)?;
            log::info!("Local player at correct position, moving to {next_pos:?}");
            arena_teams.swap_local_to_pos(next_pos).await?;
            continue;
        }

        let next_pos = arena_teams
            .get_current_position_for_player_with_given_final_position(
                arena_teams.local_player_pos(),
            )
            .ok_or(Error::PlayerMove)?;
        log::info!(
            "Moving other player to local player's current position, moving to {next_pos:?}"
        );
        arena_teams.swap_local_to_pos(next_pos).await?;
    }
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
