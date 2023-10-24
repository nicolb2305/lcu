use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Summoner {
    pub summoner_id: u64,
    pub summoner_name: String,
    pub games: u16,
}

#[derive(Deserialize, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChampionStats {
    pub champion_id: i32,
    pub wins: u16,
    pub losses: u16,
    pub kills: u32,
    pub deaths: u32,
    pub assists: u32,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    pub game_id: u64,
    pub platform_id: String,
    pub game_creation: u64,
    pub game_creation_date: String,
    pub game_duration: u32,
    pub queue_id: i32,
    pub map_id: u32,
    pub season_id: u32,
    pub game_version: String,
    pub game_mode: String,
    pub game_type: String,
    pub teams: Vec<LolMatchHistoryMatchHistoryTeam>,
    pub participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    pub participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    pub champion_id: i32,
    pub pick_turn: u32,
}

#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    pub participant_id: u32,
    pub player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Debug, Clone, Default)]
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

#[derive(Deserialize, Debug, Clone, Default)]
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
