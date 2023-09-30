use crate::{
    client::Client,
    types::{
        LolChatConversationMessageResource, LolChatConversationResource, LolChatFriendResource,
        LolLobbyLobbyChangeGameDto, LolLobbyLobbyDto, LolLobbyLobbyInvitationDto,
    },
    Error,
};

impl Client {
    pub(crate) fn get_lol_lobby_v2_lobby(&self) -> Result<LolLobbyLobbyDto, Error> {
        self.get("/lol-lobby/v2/lobby")
    }

    pub(crate) fn get_lol_chat_v1_conversations(
        &self,
    ) -> Result<Vec<LolChatConversationResource>, Error> {
        self.get("/lol-chat/v1/conversations")
    }

    pub(crate) fn post_lol_chat_v1_conversations_by_id_messages(
        &self,
        id: &str,
        body: LolChatConversationMessageResource,
    ) -> Result<LolChatConversationMessageResource, Error> {
        self.post(
            &format!("/lol-chat/v1/conversations/{id}/messages"),
            &Some(body),
        )
    }

    pub(crate) fn post_lol_lobby_v2_lobby(
        &self,
        body: LolLobbyLobbyChangeGameDto,
    ) -> Result<LolLobbyLobbyDto, Error> {
        self.post("/lol-lobby/v2/lobby", &Some(body))
    }

    pub(crate) fn get_lol_chat_v1_friends(&self) -> Result<Vec<LolChatFriendResource>, Error> {
        self.get("/lol-chat/v1/friends")
    }

    pub(crate) fn post_lol_lobby_v2_lobby_invitations(
        &self,
        body: Vec<LolLobbyLobbyInvitationDto>,
    ) -> Result<Vec<LolLobbyLobbyInvitationDto>, Error> {
        self.post("/lol-lobby/v2/lobby/invitations", &Some(body))
    }
}
