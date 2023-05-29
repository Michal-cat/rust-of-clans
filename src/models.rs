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
    tag: String,
    state: ClanWarLeagueGroupState,
    season: String,
    clans: Vec<ClanWarLeagueClan>,
    rounds: Vec<ClanWarLeagueRound>,
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
    tag: String,
    #[serde(rename = "clanLevel")]
    clan_level: i64,
    name: String,
    members: Vec<ClanWarLeagueClanMember>,
    #[serde(rename = "badgeUrls")]
    badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueClanMember {
    tag: String,
    #[serde(rename = "townHallLevel")]
    town_hall_level: i64,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLeagueRound {
    #[serde(rename = "warTags")]
    war_tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarLogEntry {
    clan: WarClan,
    #[serde(rename = "teamSize")]
    team_size: i64,
    #[serde(rename = "attacksPerMember")]
    attacks_per_member: i64,
    opponent: WarClan,
    #[serde(rename = "endTime")]
    end_time: String,
    result: ClanWarResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarClan {
    #[serde(rename = "destructionPercentage")]
    destruction_percentage: f64,
    tag: String,
    name: String,
    #[serde(rename = "badgeUrls")]
    badge_urls: HashMap<String, String>,
    #[serde(rename = "clanLevel")]
    clan_level: i64,
    attacks: i64,
    stars: i64,
    #[serde(rename = "expEarned")]
    exp_earned: i64,
    members: Vec<ClanWarMember>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarMember {
    tag: String,
    name: String,
    #[serde(rename = "mapPosition")]
    map_position: i64,
    #[serde(rename = "townhallLevel")]
    town_hall_level: i64,
    #[serde(rename = "opponentAttacks")]
    opponent_attacks: i64,
    #[serde(rename = "bestOpponentAttack")]
    best_opponent_attack: ClanWarAttack,
    attacks: Vec<ClanWarAttack>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanWarAttack {
    order: i64,
    #[serde(rename = "attackerTag")]
    attacker_tag: String,
    #[serde(rename = "defenderTag")]
    defender_tag: String,
    stars: i64,
    #[serde(rename = "destructionPercentage")]
    destruction_percentage: i64,
    duration: i64,
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
