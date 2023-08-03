use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clan {
    pub war_league: WarLeague,
    pub capital_league: CapitalLeague,
    pub member_list: Vec<ClanMember>,
    pub tag: String,
    pub clan_builder_base_points: i64,
    pub clan_versus_points: i64,
    pub war_wins: i64,
    pub war_ties: i64,
    pub war_losses: i64,
    pub clan_points: i64,
    pub required_town_hall_level: Option<i64>,
    pub chat_language: Language,
    pub is_family_friendly: bool,
    pub clan_capital_points: i64,
    pub required_trophies: i64,
    pub required_builder_base_trophies: i64,
    pub required_versus_trophies: i64,
    pub is_war_log_public: bool,
    pub war_frequency: WarFrequency,
    pub clan_level: i64,
    pub war_win_streak: i64,
    pub labels: Vec<Label>,
    pub name: String,
    pub location: Location,
    pub description: String,
    pub members: i64,
    pub r#type: ClanType,
    pub clan_capital: ClanCapital,
}

/// Represents a member of a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanMember {
    pub league: League,
    pub builder_base_league: BuilderBaseLague,
    pub versus_trophies: i64,
    pub tag: String,
    pub name: String,
    pub role: ClanMemberRole,
    pub exp_level: i64,
    pub clan_rank: i64,
    pub previous_clan_rank: i64,
    pub donations: i64,
    pub donations_received: i64,
    pub trophies: i64,
    pub builder_base_trophies: i64,
    pub player_house: Option<PlayerHouse>,
}

/// Represents a league in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub name: String,
    pub id: i64,
    pub icon_urls: HashMap<String, String>,
}

/// Represents the builder base league of a Clash of Clans player.
#[derive(Serialize, Deserialize, Debug)]
pub struct BuilderBaseLague {
    pub name: String,
    pub id: i64,
}

/// Represents the war league of a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
pub struct WarLeague {
    pub name: String,
    pub id: i64,
}

/// Represents the capital league of a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalLeague {
    pub name: String,
    pub id: i64,
}

/// Represents the player's house in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouse {
    pub elements: Vec<PlayerHouseElement>,
}

/// Represents an element in the player's house in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouseElement {
    pub id: i64,
    pub r#type: PlayerHouseElementType,
}

/// Represents the type of the Clash of Clans clan.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClanType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "inviteOnly")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

/// Represents the role of a member in a Clash of Clans clan.
#[derive(Debug, Serialize, Deserialize)]
pub enum ClanMemberRole {
    #[serde(rename = "notMember")]
    NotMember,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "leader")]
    Leader,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "coLeader")]
    CoLeader,
}

/// Represents the type of an element in the player's house in Clash of Clans.
#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerHouseElementType {
    #[serde(rename = "ground")]
    Ground,
    #[serde(rename = "roof")]
    Roof,
    #[serde(rename = "foot")]
    Foot,
    #[serde(rename = "decoration")]
    Decoration,
    #[serde(rename = "walls")]
    Walls,
}

/// Represents the language of a Clash of Clans chat.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    pub name: String,
    pub id: i64,
    pub language_code: String,
}

/// Represents the war frequency of a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
pub enum WarFrequency {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "moreThanOncePerWeek")]
    MoreThanOncePerWeek,
    #[serde(rename = "oncePerWeek")]
    OncePerWeek,
    #[serde(rename = "lessThanOncePerWeek")]
    LessThanOncePerWeek,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "any")]
    Any,
}

/// Represents a label in a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Label {
    pub name: String,
    pub id: i64,
    pub badge_urls: Option<HashMap<String, String>>,
}

/// Represents the clan capital in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapital {
    pub capital_hall_level: i64,
    pub districts: Vec<ClanDistrictData>,
}

/// Represents a district in the clan capital of Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanDistrictData {
    pub name: String,
    pub id: i64,
    pub district_hall_level: i64,
}

/// Represents the location of a Clash of Clans clan or player.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub localized_name: Option<String>,
    pub id: i64,
    pub name: String,
    pub is_country: bool,
    pub country_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueGroup {
    pub tag: Option<String>,
    pub state: ClanWarLeagueGroupState,
    pub season: Option<String>,
    pub clans: Option<Vec<ClanWarLeagueClan>>,
    pub rounds: Option<Vec<ClanWarLeagueRound>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClanWarLeagueGroupState {
    #[serde(rename = "groupNotFound")]
    GroupNotFound,
    #[serde(rename = "notInWar")]
    NotInWar,
    #[serde(rename = "preparation")]
    Preparation,
    #[serde(rename = "inWar")]
    InWar,
    #[serde(rename = "ended")]
    Ended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueClan {
    pub tag: String,
    pub clan_level: i64,
    pub name: String,
    pub members: Vec<ClanWarLeagueClanMember>,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueClanMember {
    pub tag: String,
    pub town_hall_level: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLeagueRound {
    pub war_tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarLogEntry {
    pub clan: WarClan,
    pub team_size: i64,
    pub attacks_per_member: i64,
    pub opponent: WarClan,
    pub end_time: String,
    pub result: Option<ClanWarResult>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WarClan {
    pub destruction_percentage: f64,
    pub tag: Option<String>,
    pub name: Option<String>,
    pub badge_urls: HashMap<String, String>,
    pub clan_level: i64,
    pub attacks: Option<i64>,
    pub stars: i64,
    pub exp_earned: Option<i64>,
    pub members: Option<Vec<ClanWarMember>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    pub map_position: i64,
    pub town_hall_level: i64,
    pub opponent_attacks: i64,
    pub best_opponent_attack: ClanWarAttack,
    pub attacks: Vec<ClanWarAttack>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWarAttack {
    pub order: i64,
    pub attacker_tag: String,
    pub defender_tag: String,
    pub stars: i64,
    pub destruction_percentage: i64,
    pub duration: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClanWarResult {
    #[serde(rename = "lose")]
    Lose,
    #[serde(rename = "win")]
    NotInWar,
    #[serde(rename = "tie")]
    Preparation,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanWar {
    pub clan: WarClan,
    pub team_size: Option<i64>,
    pub attacks_per_member: Option<i64>,
    pub opponent: WarClan,
    pub start_time: Option<String>,
    pub state: ClanWarState,
    pub end_time: Option<String>,
    pub preparation_start_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClanWarState {
    #[serde(rename = "clanNotFound")]
    ClanNotFound,
    #[serde(rename = "accessDenied")]
    AccessDenied,
    #[serde(rename = "notInWar")]
    NotInWar,
    #[serde(rename = "inMatchmaking")]
    InMatchMaking,
    #[serde(rename = "enterWar")]
    EnterWar,
    #[serde(rename = "matched")]
    Matched,
    #[serde(rename = "preparation")]
    Preparation,
    #[serde(rename = "war")]
    War,
    #[serde(rename = "inWar")]
    InWar,
    #[serde(rename = "ended")]
    Ended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeason {
    pub attack_log: Vec<ClanCapitalRaidSeasonAttackLogEntry>,
    pub defense_log: Vec<ClanCapitalRaidSeasonDefenseLogEntry>,
    pub state: String,
    pub start_rime: Option<String>,
    pub end_time: String,
    pub capital_total_loot: i64,
    pub raids_completed: i64,
    pub total_attacks: i64,
    pub enemy_districts_destroyed: i64,
    pub offensive_reward: i64,
    pub defensive_reward: i64,
    pub members: Option<Vec<ClanCapitalRaidSeasonMember>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonAttackLogEntry {
    pub defender: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonClanInfo {
    pub tag: String,
    pub name: String,
    pub level: i64,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonDistrict {
    pub stars: i64,
    pub name: String,
    pub id: i64,
    pub destruction_percent: i64,
    pub attack_count: i64,
    pub total_looted: i64,
    pub attacks: Option<Vec<ClanCapitalRaidSeasonAttack>>,
    pub district_hall_level: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonAttack {
    pub attacker: ClanCapitalRaidSeasonAttacker,
    pub destruction_percent: i64,
    pub stars: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonAttacker {
    pub tag: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonMember {
    pub tag: String,
    pub name: String,
    pub attacks: i64,
    pub attack_limit: i64,
    pub bonus_attack_limit: i64,
    pub capital_resources_looted: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClanCapitalRaidSeasonDefenseLogEntry {
    pub attacker: ClanCapitalRaidSeasonClanInfo,
    pub attack_count: i64,
    pub district_count: i64,
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>,
}
