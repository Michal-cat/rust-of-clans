use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::clans::models::Paging;

#[derive(Serialize, Deserialize, Debug)]
pub struct LeagueList<T> {
    items: Vec<T>,
    paging: Paging,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CapitalLeague {
    pub name: String,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub name: String,
    pub id: i64,
    pub icon_urls: Option<HashMap<String, String>>,
}
