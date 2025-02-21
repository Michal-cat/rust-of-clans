use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::clans::models::{BuilderBaseLague, ClanMemberRole, Label, League, PlayerHouse};

/// Represents a player in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub league: Option<League>,
    pub builder_base_league: Option<BuilderBaseLague>,
    pub clan: Option<PlayerClan>,
    pub role: Option<ClanMemberRole>,
    pub war_preference: WarPreference,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub versus_trophies: Option<i64>,
    pub town_hall_level: i64,
    pub town_hall_weapon_level: Option<i64>,
    pub versus_battle_wins: Option<i64>,
    pub legend_statistics: Option<PlayerLegendStatistics>,
    pub troops: Vec<PlayerItemLevel>,
    pub heroes: Vec<PlayerItemLevel>,
    pub spells: Vec<PlayerItemLevel>,
    pub labels: Vec<Label>,
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub trophies: i64,
    pub best_trophies: i64,
    pub donations: i64,
    pub donations_received: i64,
    pub builder_hall_level: i64,
    pub builder_base_trophies: i64,
    pub best_builder_base_trophies: i64,
    pub war_stars: i64,
    pub achivements: Option<Vec<PlayerAchivementProgress>>,
    pub clan_capital_contributions: i64,
    pub player_house: Option<PlayerHouse>,
}

/// Represents the clan information of a player.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerClan {
    pub tag: String,
    pub clan_level: i64,
    pub name: String,
    pub badge_urls: HashMap<String, String>,
}

/// Represents the war preference of a player.
#[derive(Serialize, Deserialize, Debug)]
pub enum WarPreference {
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "in")]
    In,
}

/// Represents the legend statistics of a player.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLegendStatistics {
    pub legend_trophies: i64,
    pub previous_builder_base_season: Option<LegendLeagueTournamentSeasonResult>,
    pub previous_versus_season: Option<LegendLeagueTournamentSeasonResult>,
    pub best_builder_base_season: Option<LegendLeagueTournamentSeasonResult>,
    pub best_versus_season: Option<LegendLeagueTournamentSeasonResult>,
    pub previous_season: Option<LegendLeagueTournamentSeasonResult>,
    pub best_season: Option<LegendLeagueTournamentSeasonResult>,
    pub current_seasob: Option<LegendLeagueTournamentSeasonResult>,
}

/// Represents the result of a legend league tournament season for a player.
#[derive(Serialize, Deserialize, Debug)]
pub struct LegendLeagueTournamentSeasonResult {
    pub trophies: i64,
    pub id: Option<String>,
    pub rank: i64,
}

/// Represents a level of a player's item (troop, hero, or spell).
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerItemLevel {
    pub level: i64,
    pub name: String,
    pub max_level: i64,
    pub village: VillageType,
    pub super_troop_is_active: Option<bool>,
}

/// Represents the type of village (home or builder base).
#[derive(Serialize, Deserialize, Debug)]
pub enum VillageType {
    #[serde(rename = "home")]
    HomeVillage,
    #[serde(rename = "builderBase")]
    BuilderBase,
}

/// Represents the progress of a player's achievement.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAchivementProgress {
    pub stars: i64,
    pub value: i64,
    pub name: String,
    pub target: i64,
    pub info: String,
    pub completion_info: String,
    pub village: VillageType,
}
