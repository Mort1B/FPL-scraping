use itertools::Itertools;
use plotters::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// use std::io; // FOR writing for excel later
// use std::process; for writing to excel later

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
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut wtr = csv::Writer::from_writer(io::stdout()); // EXCEL WRITER FOR LATER

    //attempt 100 at chart
    let root = BitMapBackend::new("plotters-net.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Points on bench", ("sans-serif", 40))
        .build_cartesian_2d(0..40, 0..150)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let team_id = 657266;
    // Base URL for player data
    let player_base = "https://fantasy.premierleague.com/api/entry/";

    let player_history = reqwest::Client::new()
        .get(player_base.to_string() + team_id.to_string().as_str() + "/history/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let mut pointsvec = Vec::new();

    let issues = serde_json::from_str::<Root>(&player_history).unwrap();
    for i in issues.current {
        // println!("{:?}, i", i);
        println!("{:?}", i.points);
        pointsvec.push(i.points_on_bench as i32)
    }

    ctx.draw_series(LineSeries::new(
        (0..).zip(pointsvec[..].iter()).map(|(idx, y)| (idx, *y)),
        &BLUE,
    ))
    .unwrap();
    root.present().unwrap();

    Ok(())
}
