use crate::{
    client::Client,
    types::{
        LolChatConversationMessageResource, LolChatConversationResource, LolChatFriendResource,
        LolLobbyGameModeDto, LolLobbyLobbyChangeGameDto, LolLobbyLobbyDto,
        LolLobbyLobbyInvitationDto, LolMatchHistoryMatchHistoryGame,
        LolMatchHistoryMatchHistoryList,
    },
    Error,
};

impl Client {
    pub(crate) async fn get_lol_lobby_v2_lobby(&self) -> Result<LolLobbyLobbyDto, Error> {
        self.get("/lol-lobby/v2/lobby", &None::<()>).await
    }

    pub(crate) async fn get_lol_chat_v1_conversations(
        &self,
    ) -> Result<Vec<LolChatConversationResource>, Error> {
        self.get("/lol-chat/v1/conversations", &None::<()>).await
    }

    pub(crate) async fn post_lol_chat_v1_conversations_by_id_messages(
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

    pub(crate) async fn post_lol_lobby_v2_lobby(
        &self,
        body: LolLobbyLobbyChangeGameDto,
    ) -> Result<LolLobbyLobbyDto, Error> {
        self.post("/lol-lobby/v2/lobby", &Some(body)).await
    }

    pub(crate) async fn get_lol_chat_v1_friends(
        &self,
    ) -> Result<Vec<LolChatFriendResource>, Error> {
        self.get("/lol-chat/v1/friends", &None::<()>).await
    }

    pub(crate) async fn post_lol_lobby_v2_lobby_invitations(
        &self,
        body: Vec<LolLobbyLobbyInvitationDto>,
    ) -> Result<Vec<LolLobbyLobbyInvitationDto>, Error> {
        self.post("/lol-lobby/v2/lobby/invitations", &Some(body))
            .await
    }

    pub(crate) async fn get_lol_match_history_v1_products_lol_current_summoner_matches(
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

    pub(crate) async fn get_lol_match_history_v1_games_by_game_id(
        &self,
        game_id: u64,
    ) -> Result<LolMatchHistoryMatchHistoryGame, Error> {
        self.get(
            &format!("/lol-match-history/v1/games/{game_id}"),
            &None::<()>,
        )
        .await
    }

    pub(crate) async fn get_lol_lobby_v1_parties_gamemode(
        &self,
    ) -> Result<LolLobbyGameModeDto, Error> {
        self.get("/lol-lobby/v1/parties/gamemode", &None::<()>)
            .await
    }
}
