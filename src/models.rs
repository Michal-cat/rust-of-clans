use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Clan {
    tag: String,
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
    is_war_log_public: bool
}
