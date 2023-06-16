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
