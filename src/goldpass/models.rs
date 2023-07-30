use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GoldPassSeason {
    pub start_time: String,
    pub end_time: String
}