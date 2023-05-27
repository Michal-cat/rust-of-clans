use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct League {
    pub name: String,
    pub id: i64,
    #[serde(rename = "iconUrls")]
    pub icon_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuilderBaseLague {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouse {
    pub elements: Vec<PlayerHouseElement>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerHouseElement {
    pub id: i64,
    pub r#type: PlayerHouseElementType,
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
    pub name: String,
    pub id: i64,
    #[serde(rename = "languageCode")]
    pub language_code: String,
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
    pub name: String,
    pub id: i64,
    #[serde(rename = "badgeUrls")]
    pub badge_urls: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanCapital {
    #[serde(rename = "capitalHallLevel")]
    pub capital_hall_level: i64,
    pub districts: Vec<ClanDistrictData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClanDistrictData {
    pub name: String,
    pub id: i64,
    #[serde(rename = "districtHallLevel")]
    pub district_hall_level: i64,
}

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
