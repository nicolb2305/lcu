#![allow(clippy::missing_errors_doc)]
use std::collections::HashMap;

use crate::{
    client::Client,
    types::{
        LolChallengesUIChallenge, LolChampSelectChampSelectAction,
        LolChampSelectChampSelectPlayerSelection, LolChampSelectChampSelectSession,
        LolChampionMasteryChampionMastery, LolChatConversationMessageResource,
        LolChatConversationResource, LolChatFriendResource, LolLobbyGameModeDto,
        LolLobbyLobbyChangeGameDto, LolLobbyLobbyDto, LolLobbyLobbyInvitationDto,
        LolLobbySubteamDataDto, LolMatchHistoryMatchHistoryGame, LolMatchHistoryMatchHistoryList,
    },
    Error,
};

impl Client {
    pub async fn get_lol_lobby_v2_lobby(&self) -> Result<LolLobbyLobbyDto, Error> {
        self.get("/lol-lobby/v2/lobby", &None::<()>).await
    }

    pub async fn get_lol_chat_v1_conversations(
        &self,
    ) -> Result<Vec<LolChatConversationResource>, Error> {
        self.get("/lol-chat/v1/conversations", &None::<()>).await
    }

    pub async fn post_lol_chat_v1_conversations_by_id_messages(
        &self,
        id: &str,
        body: LolChatConversationMessageResource,
    ) -> Result<LolChatConversationMessageResource, Error> {
        self.post(
            &format!("/lol-chat/v1/conversations/{id}/messages"),
            &Some(body),
        )
        .await
    }

    pub async fn post_lol_lobby_v2_lobby(
        &self,
        body: LolLobbyLobbyChangeGameDto,
    ) -> Result<LolLobbyLobbyDto, Error> {
        self.post("/lol-lobby/v2/lobby", &Some(body)).await
    }

    pub async fn get_lol_chat_v1_friends(&self) -> Result<Vec<LolChatFriendResource>, Error> {
        self.get("/lol-chat/v1/friends", &None::<()>).await
    }

    pub async fn post_lol_lobby_v2_lobby_invitations(
        &self,
        body: Vec<LolLobbyLobbyInvitationDto>,
    ) -> Result<Vec<LolLobbyLobbyInvitationDto>, Error> {
        self.post("/lol-lobby/v2/lobby/invitations", &Some(body))
            .await
    }

    pub async fn get_lol_match_history_v1_products_lol_current_summoner_matches(
        &self,
        beg_index: Option<u8>,
        end_index: Option<u8>,
    ) -> Result<LolMatchHistoryMatchHistoryList, Error> {
        // let params = v
        self.get(
            "/lol-match-history/v1/products/lol/current-summoner/matches",
            &Some(&[("begIndex", beg_index), ("endIndex", end_index)]),
        )
        .await
    }

    pub async fn get_lol_match_history_v1_games_by_game_id(
        &self,
        game_id: u64,
    ) -> Result<LolMatchHistoryMatchHistoryGame, Error> {
        self.get(
            &format!("/lol-match-history/v1/games/{game_id}"),
            &None::<()>,
        )
        .await
    }

    pub async fn get_lol_lobby_v1_parties_gamemode(&self) -> Result<LolLobbyGameModeDto, Error> {
        self.get("/lol-lobby/v1/parties/gamemode", &None::<()>)
            .await
    }

    pub async fn put_lol_lobby_v2_lobby_subteam_data(
        &self,
        subteam_data: &LolLobbySubteamDataDto,
    ) -> Result<(), Error> {
        self.put_empty_response("/lol-lobby/v2/lobby/subteamData", subteam_data)
            .await
    }

    pub async fn get_lol_champion_mastery_v1_local_player_champion_mastery(
        &self,
    ) -> Result<Vec<LolChampionMasteryChampionMastery>, Error> {
        self.get(
            "/lol-champion-mastery/v1/local-player/champion-mastery",
            &None::<()>,
        )
        .await
    }

    pub async fn get_lol_challenges_v1_challenges_local_player(
        &self,
    ) -> Result<HashMap<String, LolChallengesUIChallenge>, Error> {
        self.get("/lol-challenges/v1/challenges/local-player", &None::<()>)
            .await
    }

    pub async fn get_lol_champ_select_v1_session(
        &self,
    ) -> Result<LolChampSelectChampSelectSession, Error> {
        self.get("/lol-champ-select/v1/session", &None::<()>).await
    }

    pub async fn get_lol_champ_select_v1_session_my_selection(
        &self,
    ) -> Result<LolChampSelectChampSelectPlayerSelection, Error> {
        self.get("/lol-champ-select/v1/session/my-selection", &None::<()>)
            .await
    }

    pub async fn patch_lol_champ_select_v1_session_actions_by_id(
        &self,
        id: u64,
        body: LolChampSelectChampSelectAction,
    ) -> Result<(), Error> {
        self.patch_empty_response(&format!("/lol-champ-select/v1/session/actions/{id}"), &body)
            .await
    }
}
