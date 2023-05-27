use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clan {
    #[serde(rename = "warLeague")]
    war_league: WarLeague,
    #[serde(rename = "capitalLeague")]
    capital_league: CapitalLeague,
    #[serde(rename = "memberList")]
    member_list: Vec<ClanMember>,
    tag: String,
    #[serde(rename = "clanBuilderBasePoints")]
    clan_builder_base_points: i64,
    #[serde(rename = "clanVersusPoints")]
    clan_versus_points: i64,
    #[serde(rename = "warWins")]
    war_wins: i64,
    #[serde(rename = "warTies")]
    war_ties: i64,
    #[serde(rename = "warLosses")]
    war_losses: i64,
    #[serde(rename = "clanPoints")]
    clan_points: i64,
    #[serde(rename = "requiredTownhallLevel")]
    required_town_hall_level: i64,
    #[serde(rename = "chatLanguage")]
    chat_language: Language,
    #[serde(rename = "isFamilyFriendly")]
    is_family_friendly: bool,
    #[serde(rename = "clanCapitalPoints")]
    clan_capital_points: i64,
    #[serde(rename = "requiredTrophies")]
    required_trophies: i64,
    #[serde(rename = "requiredBuilderBaseTrophies")]
    required_builder_base_trophies: i64,
    #[serde(rename = "requiredVersusTrophies")]
    required_versus_trophies: i64,
    #[serde(rename = "isWarLogPublic")]
    is_war_log_public: bool,
    #[serde(rename = "warFrequency")]
    war_frequency: WarFrequency,
    #[serde(rename = "clanLevel")]
    clan_level: i64,
    #[serde(rename = "warWinStreak")]
    war_win_streak: i64,
    labels: Vec<Label>,
    name: String,
    location: Location,

    description: String,
    members: i64,
    r#type: ClanType,

    #[serde(rename = "clanCapital")]
    clan_capital: ClanCapital,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanMember {
    league: League,
    #[serde(rename = "builderBaseLeague")]
    builder_base_league: BuilderBaseLague,
    #[serde(rename = "versusTrophies")]
    versus_trophies: i64,
    tag: String,
    name: String,
    role: ClanMemberRole,
    #[serde(rename = "expLevel")]
    exp_level: i64,
    #[serde(rename = "clanRank")]
    clan_rank: i64,
    #[serde(rename = "previousClanRank")]
    previous_clan_rank: i64,
    donations: i64,
    #[serde(rename = "donationsReceived")]
    donations_received: i64,
    trophies: i64,
    #[serde(rename = "builderBaseTrophies")]
    builder_base_trophies: i64,
    #[serde(rename = "playerHouse")]
    player_house: Option<PlayerHouse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct League {
    name: String,
    id: i64,
    #[serde(rename = "iconUrls")]
    icon_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuilderBaseLague {
    name: String,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarLeague {
    name: String,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalLeague {
    name: String,
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouse {
    elements: Vec<PlayerHouseElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouseElement {
    id: i64,
    r#type: PlayerHouseElementType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClanType {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "invite_only")]
    InviteOnly,
    #[serde(rename = "closed")]
    Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClanMemberRole {
    #[serde(rename = "not_member")]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Language {
    name: String,
    id: i64,
    #[serde(rename = "languageCode")]
    language_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WarFrequency {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "more_than_once_per_week")]
    MoreThanOncePerWeek,
    #[serde(rename = "once_per_week")]
    OncePerWeek,
    #[serde(rename = "less_than_once_per_week")]
    LessThanOncePerWeek,
    #[serde(rename = "never")]
    Never,
    #[serde(rename = "any")]
    Any,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Label {
    name: String,
    id: i64,
    #[serde(rename = "badgeUrls")]
    badge_urls: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapital {
    #[serde(rename = "capitalHallLevel")]
    capital_hall_level: i64,
    districts: Vec<ClanDistrictData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanDistrictData {
    name: String,
    id: i64,
    #[serde(rename = "districtHallLevel")]
    district_hall_level: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    #[serde(rename = "localizedName")]
    localized_name: Option<String>,
    id: i64,
    name: String,
    #[serde(rename = "isCountry")]
    is_country: bool,
    #[serde(rename = "countryCode")]
    country_code: String,
}
