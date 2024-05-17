use crate::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    pub error_code: String,
    pub http_status: u32,
    pub message: String,
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}): {}",
            self.error_code, self.http_status, self.message
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ApiResult<T> {
    Ok(T),
    Err(ApiError),
}

impl<T> From<ApiResult<T>> for Result<T, Error> {
    fn from(value: ApiResult<T>) -> Self {
        match value {
            ApiResult::Ok(val) => Ok(val),
            ApiResult::Err(e) => Err(Error::ApiError(e)),
        }
    }
}

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
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
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
    pub subteam_index: Option<usize>,
    pub intra_subteam_position: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq)]
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
    TooManyIncompleteSubteamsRestriction,
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

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryList {
    pub platform_id: String,
    pub account_id: u64,
    pub games: LolMatchHistoryMatchHistoryGameList,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGameList {
    pub game_index_begin: u64,
    pub game_index_end: u64,
    pub game_begin_date: String,
    pub game_end_date: String,
    pub game_count: u64,
    pub games: Vec<LolMatchHistoryMatchHistoryGame>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    pub game_id: u64,
    pub platform_id: String,
    pub game_creation: u64,
    pub game_creation_date: String,
    pub game_duration: u32,
    pub queue_id: i32,
    pub map_id: u32,
    pub season_id: Option<u32>,
    pub game_version: String,
    pub game_mode: String,
    pub game_type: String,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    pub participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeam {
    pub team_id: u32,
    pub win: String,
    pub first_blood: bool,
    pub first_tower: bool,
    pub first_inhibitor: bool,
    pub first_baron: bool,
    pub first_dargon: bool,
    pub tower_kills: u32,
    pub inhibitor_kills: u32,
    pub baron_kills: u32,
    pub dragon_kills: u32,
    pub vilemaw_kills: u32,
    pub rift_herald_kills: u32,
    pub dominion_victory_score: u32,
    pub bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    pub champion_id: i32,
    pub pick_turn: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipant {
    pub participant_id: u32,
    pub team_id: u32,
    pub champion_id: i32,
    pub spell1_id: u32,
    pub spell2_id: u32,
    pub highest_achieved_season_tier: String,
    pub stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    pub timeline: LolMatchHistoryMatchHistoryTimeline,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    pub participant_id: u32,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    pub participant_id: u32,
    pub win: bool,
    pub item0: u32,
    pub item1: u32,
    pub item2: u32,
    pub item3: u32,
    pub item4: u32,
    pub item5: u32,
    pub item6: u32,
    pub kills: i64,
    pub deaths: i64,
    pub assists: i64,
    pub largest_killing_spree: i64,
    pub largest_multi_kill: i64,
    pub killing_sprees: i64,
    pub longest_time_spent_living: i64,
    pub double_kills: i64,
    pub triple_kills: i64,
    pub quadra_kills: i64,
    pub penta_kills: i64,
    pub unreal_kills: i64,
    pub total_damage_dealt: i64,
    pub magic_damage_dealt: i64,
    pub physical_damage_dealt: i64,
    pub true_damage_dealt: i64,
    pub largest_critical_strike: i64,
    pub total_damage_dealt_to_champions: i64,
    pub magic_damage_dealt_to_champions: i64,
    pub physical_damage_dealt_to_champions: i64,
    pub true_damage_dealt_to_champions: i64,
    pub total_heal: i64,
    pub total_units_healed: i64,
    pub total_damage_taken: i64,
    pub magical_damage_taken: i64,
    pub physical_damage_taken: i64,
    pub true_damage_taken: i64,
    pub gold_earned: i64,
    pub gold_spent: i64,
    pub turret_kills: i64,
    pub inhibitor_kills: i64,
    pub total_minions_killed: i64,
    pub neutral_minions_killed: i64,
    pub neutral_minions_killed_team_jungle: i64,
    pub neutral_minions_killed_enemy_jungle: i64,
    pub total_time_crowd_control_dealt: i64,
    pub champ_level: i64,
    pub vision_wards_bought_in_game: i64,
    pub sight_wards_bought_in_game: i64,
    pub wards_placed: i64,
    pub wards_killed: i64,
    pub first_blood_kill: bool,
    pub first_blood_assist: bool,
    pub first_tower_kill: bool,
    pub first_tower_assist: bool,
    pub first_inhibitor_kill: bool,
    pub first_inhibitor_assist: bool,
    pub game_ended_in_early_surrender: bool,
    pub game_ended_in_surrender: bool,
    pub caused_early_surrender: bool,
    pub early_surrender_accomplice: bool,
    pub team_early_surrendered: bool,
    pub combat_player_score: i64,
    pub objective_player_score: i64,
    pub total_player_score: i64,
    pub total_score_rank: i64,
    pub damage_self_mitigated: i64,
    pub damage_dealt_to_objectives: i64,
    pub damage_dealt_to_turrets: i64,
    pub vision_score: i64,
    pub time_c_cing_others: i64,
    pub player_score0: i64,
    pub player_score1: i64,
    pub player_score2: i64,
    pub player_score3: i64,
    pub player_score4: i64,
    pub player_score5: i64,
    pub player_score6: i64,
    pub player_score7: i64,
    pub player_score8: i64,
    pub player_score9: i64,
    pub perk_primary_style: i64,
    pub perk_sub_style: i64,
    pub perk0: i64,
    pub perk0_var1: i64,
    pub perk0_var2: i64,
    pub perk0_var3: i64,
    pub perk1: i64,
    pub perk1_var1: i64,
    pub perk1_var2: i64,
    pub perk1_var3: i64,
    pub perk2: i64,
    pub perk2_var1: i64,
    pub perk2_var2: i64,
    pub perk2_var3: i64,
    pub perk3: i64,
    pub perk3_var1: i64,
    pub perk3_var2: i64,
    pub perk3_var3: i64,
    pub perk4: i64,
    pub perk4_var1: i64,
    pub perk4_var2: i64,
    pub perk4_var3: i64,
    pub perk5: i64,
    pub perk5_var1: i64,
    pub perk5_var2: i64,
    pub perk5_var3: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimeline {
    pub participant_id: u32,
    pub role: String,
    pub lane: String,
    pub creeps_per_min_deltas: HashMap<String, f64>,
    pub xp_per_min_deltas: HashMap<String, f64>,
    pub gold_per_min_deltas: HashMap<String, f64>,
    pub cs_diff_per_min_deltas: HashMap<String, f64>,
    pub xp_diff_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_per_min_deltas: HashMap<String, f64>,
    pub damage_taken_diff_per_min_deltas: HashMap<String, f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    pub platform_id: String,
    pub account_id: u64,
    pub summoner_id: u64,
    pub summoner_name: String,
    pub current_platform_id: String,
    pub current_account_id: u64,
    pub match_history_uri: String,
    pub profile_icon: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameModeDto {
    pub game_type: String,
    pub max_team_size: i32,
    pub max_party_size: i32,
    pub bot_difficulty: Option<String>,
    pub queue_id: Option<i32>,
    pub game_customization: Option<HashMap<String, String>>,
    pub customs_settings: Option<LolLobbyCustomGameSettingsDto>,
    pub game_type_config_id: Option<i64>,
    pub map_id: Option<i32>,
    pub allow_spectators: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameSettingsDto {
    pub lobby_name: String,
    pub lobby_password: String,
    pub game_id: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq, Eq, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySubteamDataDto {
    pub subteam_index: usize,
    pub intra_subteam_position: usize,
}
