use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyDto {
    pub party_id: String,
    pub party_type: String,
    pub members: Vec<LolLobbyLobbyParticipantDto>,
    pub local_member: LolLobbyLobbyParticipantDto,
    pub invitations: Vec<LolLobbyLobbyInvitationDto>,
    pub can_start_activity: bool,
    pub restrictions: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub warnings: Option<Vec<LolLobbyEligibilityRestriction>>,
    pub game_config: LolLobbyLobbyGameConfigDto,
    pub multi_user_chat_id: String,
    pub multi_user_chat_password: String,
    // pub multi_user_chat_j_w_t: String,
    pub muc_jwt_dto: LolLobbyMucJwtDto,
    pub scarce_positions: Vec<String>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyParticipantDto {
    pub summoner_id: u64,
    pub summoner_icon_id: i32,
    pub summoner_name: String,
    pub summoner_internal_name: String,
    pub puuid: String,
    pub summoner_level: u32,
    pub allowed_start_activity: bool,
    pub allowed_change_activity: bool,
    pub allowed_toggle_invite: bool,
    pub allowed_kick_others: bool,
    pub allowed_invite_others: bool,
    pub is_leader: bool,
    pub is_spectator: bool,
    pub team_id: i32,
    pub first_position_preference: String,
    pub second_position_preference: String,
    pub ready: bool,
    pub show_ghosted_banner: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub is_bot: bool,
    pub bot_id: String,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolLobbyLobbyBotDifficulty {
    #[default]
    #[serde(rename = "NONE")]
    None = -1,
    #[serde(rename = "EASY")]
    Easy = 0,
    #[serde(rename = "MEDIUM")]
    Medium = 1,
    #[serde(rename = "HARD")]
    Hard = 2,
    #[serde(rename = "UBER")]
    Uber = 3,
    #[serde(rename = "TUTORIAL")]
    Tutorial = 4,
    #[serde(rename = "INTRO")]
    Intro = 5,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitationDto {
    pub invitation_id: String,
    pub to_summoner_id: u64,
    pub state: LolLobbyLobbyInvitationState,
    pub timestamp: String,
    pub to_summoner_name: String,
    pub invitation_type: LolLobbyInvitationType,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolLobbyLobbyInvitationState {
    #[default]
    Requested = 0,
    Pending = 1,
    Accepted = 2,
    Joined = 3,
    Declined = 4,
    Kicked = 5,
    OnHold = 6,
    Error = 7,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolLobbyInvitationType {
    #[default]
    #[serde(rename = "invalid")]
    Invalid = 0,
    #[serde(rename = "lobby")]
    Lobby = 1,
    #[serde(rename = "party")]
    Party = 2,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibilityRestriction {
    pub restriction_code: LolLobbyEligibilityRestrictionCode,
    pub restriction_args: HashMap<String, String>,
    pub expired_timestamp: u64,
    pub summoner_ids: Vec<u64>,
    pub summoner_ids_string: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolLobbyEligibilityRestrictionCode {
    #[default]
    QueueDisabled = 0,
    QueueUnsupported = 1,
    PlayerLevelRestriction = 2,
    PlayerTimedRestriction = 3,
    PlayerBannedRestriction = 4,
    PlayerAvailableChampionRestriction = 5,
    TeamDivisionRestriction = 6,
    TeamSkillRestriction = 7,
    TeamMaxSizeRestriction = 8,
    TeamMinSizeRestriction = 9,
    PlayerBingeRestriction = 10,
    PlayerDodgeRestriction = 11,
    PlayerInGameRestriction = 12,
    PlayerLeaverBustedRestriction = 13,
    PlayerLeaverQueueLockoutRestriction = 14,
    PlayerLeaverTaintedWarningRestriction = 15,
    PlayerMaxLevelRestriction = 16,
    PlayerMinLevelRestriction = 17,
    PlayerMinorRestriction = 18,
    PlayerTimePlayedRestriction = 19,
    PlayerRankSoloOnlyRestriction = 20,
    PlayerRankedSuspensionRestriction = 21,
    TeamHighMMRMaxSizeRestriction = 22,
    TeamSizeRestriction = 23,
    PrerequisiteQueuesNotPlayedRestriction = 24,
    GameVersionMismatch = 25,
    GameVersionMissing = 26,
    GameVersionNotSupported = 27,
    QueueEntryNotEntitledRestriction = 28,
    UnknownRestriction = 29,
    BanInfoNotAvailable = 30,
    MinorInfoNotAvailable = 31,
    SummonerInfoNotAvailable = 32,
    LeaguesInfoNotAvailable = 33,
    InventoryChampsInfoNotAvailable = 34,
    InventoryQueuesInfoNotAvailable = 35,
    MmrStandardDeviationTooLarge = 36,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyGameConfigDto {
    pub game_mode: String,
    pub map_id: i32,
    pub queue_id: i32,
    pub pick_type: String,
    pub max_team_size: i32,
    pub max_lobby_size: i32,
    pub max_human_players: i32,
    pub allowable_premade_sizes: Vec<i32>,
    pub premade_size_allowed: bool,
    pub is_team_builder_managed: bool,
    pub is_custom: bool,
    pub show_position_selector: bool,
    pub is_lobby_full: bool,
    pub should_force_scarce_position_selection: bool,
    pub custom_lobby_name: String,
    pub custom_mutator_name: String,
    pub custom_team100: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_team200: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectators: Vec<LolLobbyLobbyParticipantDto>,
    pub custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub custom_rewards_disabled_reasons: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyMucJwtDto {
    pub jwt: String,
    pub channel_claim: String,
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    #[default]
    NotAllowed = 0,
    LobbyAllowed = 1,
    FriendsAllowed = 2,
    AllAllowed = 3,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationResource {
    pub id: String,
    pub name: String,
    pub pid: String,
    pub game_name: String,
    pub game_tag: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub inviter_id: String,
    pub password: String,
    // pub multi_user_chat_j_w_t: String,
    pub target_region: String,
    pub is_muted: bool,
    pub unread_message_count: u64,
    pub last_message: Option<LolChatConversationMessageResource>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationMessageResource {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub from_summoner_id: u64,
    pub from_id: String,
    pub from_pid: String,
    pub from_obfuscated_summoner_id: u64,
    pub body: String,
    pub timestamp: String,
    pub is_historical: bool,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueGameTypeConfig {
    pub id: i64,
    pub name: String,
    pub max_allowable_bans: i32,
    pub allow_trades: bool,
    pub exclusive_pick: bool,
    pub duplicate_pick: bool,
    pub team_champion_pool: bool,
    pub cross_team_champion_pool: bool,
    pub advanced_learning_quests: bool,
    pub battle_boost: bool,
    pub death_match: bool,
    pub do_not_remove: bool,
    pub learning_quests: bool,
    pub onboard_coop_beginner: bool,
    pub reroll: bool,
    pub main_pick_timer_duration: i32,
    pub post_pick_timer_duration: i32,
    pub ban_timer_duration: i32,
    pub pick_mode: String,
    pub ban_mode: String,
    pub game_mode_override: Option<String>,
    pub num_players_per_team_override: Option<i32>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    pub map_id: i32,
    pub game_mode: String,
    pub mutators: LolLobbyQueueGameTypeConfig,
    pub game_type_config: LolLobbyQueueGameTypeConfig,
    pub spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    pub team_size: i32,
    pub max_player_count: u32,
    pub tournament_game_mode: String,
    pub tournament_passback_url: String,
    pub tournament_passback_data_packet: String,
    pub game_server_region: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    pub lobby_name: String,
    pub lobby_password: String,
    pub configuration: LolLobbyLobbyCustomGameConfiguration,
    pub team_one: Vec<LolLobbyLobbyMember>,
    pub team_two: Vec<LolLobbyLobbyMember>,
    pub spectators: Vec<LolLobbyLobbyMember>,
    pub practice_game_rewards_disabled_reasons: Vec<String>,
    pub game_id: u64,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMember {
    pub id: u64,
    pub is_owner: bool,
    pub is_spectator: bool,
    pub can_invite_others: bool,
    pub position_preferences: LolLobbyLobbyPositionPreferences,
    pub excluded_position_preference: Option<String>,
    pub summoner_internal_name: String,
    pub show_position_excluder: bool,
    pub auto_fill_eligible: bool,
    pub auto_fill_protected_for_streaking: bool,
    pub auto_fill_protected_for_promos: bool,
    pub auto_fill_protected_for_soloing: bool,
    pub is_bot: bool,
    pub bot_difficulty: LolLobbyLobbyBotDifficulty,
    pub bot_champion_id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPositionPreferences {
    pub first_preference: String,
    pub second_preference: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeGameDto {
    pub queue_id: i32,
    pub is_custom: bool,
    pub custom_game_lobby: Option<LolLobbyLobbyCustomGameLobby>,
    pub game_customization: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    pub summoner_id: u64,
    pub id: String,
    pub name: String,
    pub pid: String,
    pub puuid: String,
    pub game_name: String,
    pub game_tag: String,
    pub icon: i32,
    pub availability: String,
    pub platform_id: String,
    pub patchline: String,
    pub product: String,
    pub product_name: String,
    pub summary: String,
    pub time: u64,
    pub status_message: String,
    pub note: String,
    pub last_seen_online_timestamp: Option<String>,
    pub is_p2_p_conversation_muted: bool,
    pub group_id: u32,
    pub display_group_id: u32,
    pub group_name: String,
    pub display_group_name: String,
    pub lol: HashMap<String, String>,
}
