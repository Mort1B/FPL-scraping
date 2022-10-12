use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "new_entries")]
    pub new_entries: NewEntries,
    #[serde(rename = "last_updated_data")]
    pub last_updated_data: String,
    pub league: League,
    pub standings: Standings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewEntries {
    #[serde(rename = "has_next")]
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: i64,
    pub name: String,
    pub created: String,
    pub closed: bool,
    #[serde(rename = "max_entries")]
    pub max_entries: Value,
    #[serde(rename = "league_type")]
    pub league_type: String,
    pub scoring: String,
    #[serde(rename = "admin_entry")]
    pub admin_entry: i64,
    #[serde(rename = "start_event")]
    pub start_event: i64,
    #[serde(rename = "code_privacy")]
    pub code_privacy: String,
    #[serde(rename = "has_cup")]
    pub has_cup: bool,
    #[serde(rename = "cup_league")]
    pub cup_league: Value,
    pub rank: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Standings {
    #[serde(rename = "has_next")]
    pub has_next: bool,
    pub page: i64,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub id: i64,
    #[serde(rename = "event_total")]
    pub event_total: i64,
    #[serde(rename = "player_name")]
    pub player_name: String,
    pub rank: i64,
    #[serde(rename = "last_rank")]
    pub last_rank: i64,
    #[serde(rename = "rank_sort")]
    pub rank_sort: i64,
    pub total: i64,
    pub entry: i64,
    #[serde(rename = "entry_name")]
    pub entry_name: String,
}
