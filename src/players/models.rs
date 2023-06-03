use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::clans::models::{BuilderBaseLague, ClanMemberRole, Label, League, PlayerHouse};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub league: League,
    #[serde(rename = "builderBaseLeague")]
    pub builder_base_league: BuilderBaseLague,
    pub clan: PlayerClan,
    pub role: ClanMemberRole,
    #[serde(rename = "warPreference")]
    pub war_preference: WarPreference,
    #[serde(rename = "attackWins")]
    pub attack_wins: i64,
    #[serde(rename = "defenseWins")]
    pub defense_wins: i64,
    #[serde(rename = "versusTrophies")]
    pub versus_trophies: i64,
    #[serde(rename = "townHallLevel")]
    pub town_hall_level: i64,
    #[serde(rename = "townHallWeaponLevel")]
    pub town_hall_weapon_level: i64,
    #[serde(rename = "versusBattleWins")]
    pub versus_battle_wins: i64,
    #[serde(rename = "legendStatistics")]
    pub legend_statistics: PlayerLegendStatistics,
    pub troops: Vec<PlayerItemLevel>,
    pub heroes: Vec<PlayerItemLevel>,
    pub spells: Vec<PlayerItemLevel>,
    pub labels: Vec<Label>,
    pub tag: String,
    pub name: String,
    #[serde(rename = "expLevel")]
    pub exp_level: i64,
    pub trophies: i64,
    #[serde(rename = "bestTrophies")]
    pub best_trophies: i64,
    pub donations: i64,
    #[serde(rename = "donationsReceived")]
    pub donations_received: i64,
    #[serde(rename = "builderHallLevel")]
    pub builder_hall_level: i64,
    #[serde(rename = "builderBaseTrophies")]
    pub builder_base_trophies: i64,
    #[serde(rename = "bestBuilderBaseTrophies")]
    pub best_builder_base_trophies: i64,
    #[serde(rename = "warStars")]
    pub war_stars: i64,
    pub achivements: Option<Vec<PlayerAchivementProgress>>,
    #[serde(rename = "clanCapitalContributions")]
    pub clan_capital_contributions: i64,
    #[serde(rename = "playerHouse")]
    pub player_house: Option<PlayerHouse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerClan {
    pub tag: String,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    pub name: String,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WarPreference {
    #[serde(rename = "out")]
    Out,
    #[serde(rename = "in")]
    In,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerLegendStatistics {
    #[serde(rename = "legendTrophies")]
    pub legend_trophies: i64,
    #[serde(rename = "previousBuilderBaseSeason")]
    pub previous_builder_base_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "previousVersusSeason")]
    pub previous_versus_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "bestBuilderBaseSeason")]
    pub best_builder_base_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "bestVersusSeason")]
    pub best_versus_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "previousSeason")]
    pub previous_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "bestSeason")]
    pub best_season: Option<LegendLeagueTournamentSeasonResult>,
    #[serde(rename = "currentSeason")]
    pub current_seasob: Option<LegendLeagueTournamentSeasonResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LegendLeagueTournamentSeasonResult {
    pub trophies: i64,
    pub id: Option<String>,
    pub rank: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerItemLevel {
    pub level: i64,
    pub name: String,
    #[serde(rename = "maxLevel")]
    pub max_level: i64,
    pub village: VillageType,
    #[serde(rename = "superTroopIsActive")]
    pub super_troop_is_active: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum VillageType {
    #[serde(rename = "home")]
    HomeVillage,
    #[serde(rename = "builderBase")]
    BuilderBase,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerAchivementProgress {
    pub stars: i64,
    pub value: i64,
    pub name: String,
    pub target: i64,
    pub info: String,
    #[serde(rename = "completionInfo")]
    pub completion_info: String,
    pub village: VillageType,
}
