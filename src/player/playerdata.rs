use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub current: Vec<Current>,
    pub past: Vec<Past>,
    pub chips: Vec<Chip>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub event: i64,
    pub points: i64,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    pub rank: i64,
    #[serde(rename = "rank_sort")]
    pub rank_sort: i64,
    #[serde(rename = "overall_rank")]
    pub overall_rank: i64,
    pub bank: i64,
    pub value: i64,
    #[serde(rename = "event_transfers")]
    pub event_transfers: i64,
    #[serde(rename = "event_transfers_cost")]
    pub event_transfers_cost: i64,
    #[serde(rename = "points_on_bench")]
    pub points_on_bench: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Past {
    #[serde(rename = "season_name")]
    pub season_name: String,
    #[serde(rename = "total_points")]
    pub total_points: i64,
    pub rank: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chip {
    pub name: String,
    pub time: String,
    pub event: i64,
}
