use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
pub struct Clan {
    #[serde(rename = "warLeague")]
    pub war_league: WarLeague,
    #[serde(rename = "capitalLeague")]
    pub capital_league: CapitalLeague,
    #[serde(rename = "memberList")]
    pub member_list: Vec<ClanMember>,
    pub tag: String,
    #[serde(rename = "clanBuilderBasePoints")]
    pub clan_builder_base_points: i64,
    #[serde(rename = "clanVersusPoints")]
    pub clan_versus_points: i64,
    #[serde(rename = "warWins")]
    pub war_wins: i64,
    #[serde(rename = "warTies")]
    pub war_ties: i64,
    #[serde(rename = "warLosses")]
    pub war_losses: i64,
    #[serde(rename = "clanPoints")]
    pub clan_points: i64,
    #[serde(rename = "requiredTownhallLevel")]
    pub required_town_hall_level: i64,
    #[serde(rename = "chatLanguage")]
    pub chat_language: Language,
    #[serde(rename = "isFamilyFriendly")]
    pub is_family_friendly: bool,
    #[serde(rename = "clanCapitalPoints")]
    pub clan_capital_points: i64,
    #[serde(rename = "requiredTrophies")]
    pub required_trophies: i64,
    #[serde(rename = "requiredBuilderBaseTrophies")]
    pub required_builder_base_trophies: i64,
    #[serde(rename = "requiredVersusTrophies")]
    pub required_versus_trophies: i64,
    #[serde(rename = "isWarLogPublic")]
    pub is_war_log_public: bool,
    #[serde(rename = "warFrequency")]
    pub war_frequency: WarFrequency,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    #[serde(rename = "warWinStreak")]
    pub war_win_streak: i64,
    pub labels: Vec<Label>,
    pub name: String,
    pub location: Location,
    pub description: String,
    pub members: i64,
    pub r#type: ClanType,
    #[serde(rename = "clanCapital")]
    pub clan_capital: ClanCapital,
}

/// Represents a member of a Clash of Clans clan.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClanMember {
    pub league: League,
    #[serde(rename = "builderBaseLeague")]
    pub builder_base_league: BuilderBaseLague,
    #[serde(rename = "versusTrophies")]
    pub versus_trophies: i64,
    pub tag: String,
    pub name: String,
    pub role: ClanMemberRole,
    #[serde(rename = "expLevel")]
    pub exp_level: i64,
    #[serde(rename = "clanRank")]
    pub clan_rank: i64,
    #[serde(rename = "previousClanRank")]
    pub previous_clan_rank: i64,
    pub donations: i64,
    #[serde(rename = "donationsReceived")]
    pub donations_received: i64,
    pub trophies: i64,
    #[serde(rename = "builderBaseTrophies")]
    pub builder_base_trophies: i64,
    #[serde(rename = "playerHouse")]
    pub player_house: Option<PlayerHouse>,
}

/// Represents a league in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
pub struct League {
    pub name: String,
    pub id: i64,
    #[serde(rename = "iconUrls")]
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
pub struct Language {
    pub name: String,
    pub id: i64,
    #[serde(rename = "languageCode")]
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
pub struct Label {
    pub name: String,
    pub id: i64,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: Option<HashMap<String, String>>,
}

/// Represents the clan capital in Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapital {
    #[serde(rename = "capitalHallLevel")]
    pub capital_hall_level: i64,
    pub districts: Vec<ClanDistrictData>,
}

/// Represents a district in the clan capital of Clash of Clans.
#[derive(Serialize, Deserialize, Debug)]
pub struct ClanDistrictData {
    pub name: String,
    pub id: i64,
    #[serde(rename = "districtHallLevel")]
    pub district_hall_level: i64,
}

/// Represents the location of a Clash of Clans clan or player.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "localizedName")]
    pub localized_name: Option<String>,
    pub id: i64,
    pub name: String,
    #[serde(rename = "isCountry")]
    pub is_country: bool,
    #[serde(rename = "countryCode")]
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
    #[serde(rename = "war")]
    War,
    #[serde(rename = "ended")]
    Ended,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueClan {
    pub tag: String,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    pub name: String,
    pub members: Vec<ClanWarLeagueClanMember>,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueClanMember {
    pub tag: String,
    #[serde(rename = "townHallLevel")]
    pub town_hall_level: i64,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueRound {
    #[serde(rename = "warTags")]
    pub war_tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLog {
    pub items: Vec<ClanWarLogEntry>,
    pub paging: Option<Paging>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Paging {
    pub cursors: Cursors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cursors {
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLogEntry {
    pub clan: WarClan,
    #[serde(rename = "teamSize")]
    pub team_size: i64,
    #[serde(rename = "attacksPerMember")]
    pub attacks_per_member: i64,
    pub opponent: WarClan,
    #[serde(rename = "endTime")]
    pub end_time: String,
    pub result: Option<ClanWarResult>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarClan {
    #[serde(rename = "destructionPercentage")]
    pub destruction_percentage: f64,
    pub tag: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: HashMap<String, String>,
    #[serde(rename = "clanLevel")]
    pub clan_level: i64,
    pub attacks: Option<i64>,
    pub stars: i64,
    #[serde(rename = "expEarned")]
    pub exp_earned: Option<i64>,
    pub members: Option<Vec<ClanWarMember>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarMember {
    pub tag: String,
    pub name: String,
    #[serde(rename = "mapPosition")]
    pub map_position: i64,
    #[serde(rename = "townhallLevel")]
    pub town_hall_level: i64,
    #[serde(rename = "opponentAttacks")]
    pub opponent_attacks: i64,
    #[serde(rename = "bestOpponentAttack")]
    pub best_opponent_attack: ClanWarAttack,
    pub attacks: Vec<ClanWarAttack>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarAttack {
    pub order: i64,
    #[serde(rename = "attackerTag")]
    pub attacker_tag: String,
    #[serde(rename = "defenderTag")]
    pub defender_tag: String,
    pub stars: i64,
    #[serde(rename = "destructionPercentage")]
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
pub struct ClanWar {
    pub clan: WarClan,
    #[serde(rename = "teamSize")]
    pub team_size: Option<i64>,
    #[serde(rename = "attacksPerMember")]
    pub attacks_per_member: Option<i64>,
    pub opponent: WarClan,
    #[serde(rename = "startTime")]
    pub start_time: Option<String>,
    pub state: ClanWarState,
    #[serde(rename = "endTime")]
    pub end_time: Option<String>,
    #[serde(rename = "preparationStartTime")]
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
pub struct ClanMembers {
    pub items: Vec<ClanMember>,
    pub paging: Option<Paging>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalRaidSeasons {
    pub items: Vec<ClanCapitalRaidSeason>,
    pub paging: Option<Paging>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeason {
    #[serde(rename = "attackLog")]
    pub attack_log: Vec<ClanCapitalRaidSeasonAttackLogEntry>,
    #[serde(rename = "defenseLog")]
    pub defense_log: Vec<ClanCapitalRaidSeasonDefenseLogEntry>,
    pub state: String,
    #[serde(rename = "startTime")]
    pub start_rime: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "capitalTotalLoot")]
    pub capital_total_loot: i64,
    #[serde(rename = "raidsCompleted")]
    pub raids_completed: i64,
    #[serde(rename = "totalAttacks")]
    pub total_attacks: i64,
    #[serde(rename = "enemyDistrictsDestroyed")]
    pub enemy_districts_destroyed: i64,
    #[serde(rename = "offensiveReward")]
    pub offensive_reward: i64,
    #[serde(rename = "defensiveReward")]
    pub defensive_reward: i64,
    pub members: Option<Vec<ClanCapitalRaidSeasonMember>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonAttackLogEntry {
    pub defender: ClanCapitalRaidSeasonClanInfo,
    #[serde(rename = "attackCount")]
    pub attack_count: i64,
    #[serde(rename = "districtCount")]
    pub district_count: i64,
    #[serde(rename = "districtsDestroyed")]
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonClanInfo {
    pub tag: String,
    pub name: String,
    pub level: i64,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonDistrict {
    pub stars: i64,
    pub name: String,
    pub id: i64,
    #[serde(rename = "destructionPercent")]
    pub destruction_percent: i64,
    #[serde(rename = "attackCount")]
    pub attack_count: i64,
    #[serde(rename = "totalLooted")]
    pub total_looted: i64,
    pub attacks: Option<Vec<ClanCapitalRaidSeasonAttack>>,
    #[serde(rename = "districtHallLevel")]
    pub district_hall_level: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonAttack {
    pub attacker: ClanCapitalRaidSeasonAttacker,
    #[serde(rename = "destructionPercent")]
    pub destruction_percent: i64,
    pub stars: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonAttacker {
    pub tag: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonMember {
    pub tag: String,
    pub name: String,
    pub attacks: i64,
    #[serde(rename = "attackLimit")]
    pub attack_limit: i64,
    #[serde(rename = "bonusAttackLimit")]
    pub bonus_attack_limit: i64,
    #[serde(rename = "capitalResourcesLooted")]
    pub capital_resources_looted: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapitalRaidSeasonDefenseLogEntry {
    pub attacker: ClanCapitalRaidSeasonClanInfo,
    #[serde(rename = "attackCount")]
    pub attack_count: i64,
    #[serde(rename = "districtCount")]
    pub district_count: i64,
    #[serde(rename = "districtsDestroyed")]
    pub districts_destroyed: i64,
    pub districts: Vec<ClanCapitalRaidSeasonDistrict>,
}
