use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::clans::models::Paging;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueList<T> {
    pub items: Vec<T>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonList {
    pub items: Vec<SeasonInfo>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRankingList {
    pub items: Vec<PlayerRanking>,
    pub paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRanking {
    pub league: Option<League>,
    pub clan: PlayerRankingClan,
    pub attack_wins: i64,
    pub defense_wins: i64,
    pub tag: String,
    pub name: String,
    pub exp_level: i64,
    pub rank: i64,
    pub previous_rank: Option<i64>,
    pub trophies: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRankingClan {
    pub tag: String,
    pub name: String,
    pub badge_urls: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub name: String,
    pub id: u32,
    pub icon_urls: Option<HashMap<String, String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueInfo {
    pub id: u32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SeasonInfo {
    pub id: String,
}
