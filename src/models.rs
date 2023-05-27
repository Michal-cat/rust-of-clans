use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clan {
    tag: String,
    name: String,
    description: String,
    members: i64,
    r#type: ClanType,
    #[serde(rename = "warLeague")]
    war_league: WarLeague,
    #[serde(rename = "capitalLeague")]
    capital_league: CapitalLeague,
    #[serde(rename = "memberList")]
    member_list: Vec<ClanMember>,
    #[serde(rename = "clanLevel")]
    clan_level: i64,
    #[serde(rename = "warWinStreak")]
    war_win_streak: i64,
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
    #[serde(rename = "isFamilyFriendly")]
    is_family_friendly: bool,
    #[serde(rename = "clanBuilderBasePoints")]
    clan_builder_base_points: i64,
    #[serde(rename = "clanVersusPoints")]
    clan_versus_points: i64,
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
    player_house: PlayerHouse,
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
    
}