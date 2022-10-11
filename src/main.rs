use itertools::Itertools;
use plotters::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
// use std::io; // FOR writing for excel later
// use std::process; for writing to excel later

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut wtr = csv::Writer::from_writer(io::stdout()); // EXCEL WRITER FOR LATER

    let team_id = 657266;

    // Getting League info from API
    let league = reqwest::Client::new()
        .get("https://fantasy.premierleague.com/api/leagues-classic/41686/standings/")
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&league).unwrap();
    let players = json.get("standings").unwrap().get("results").unwrap();

    // Base URL for player data
    let player_base = "https://fantasy.premierleague.com/api/entry/";

    // data storage league data
    let mut by_points = HashMap::new();
    let mut by_position = HashMap::new();

    for player in players.as_array() {
        for x in player {
            let previous_pos = x.get("last_rank").unwrap().as_i64().unwrap();
            let this_pos = x.get("rank_sort").unwrap().as_i64().unwrap();
            let spiller = x.get("player_name").unwrap().as_str().unwrap();
            let total = x.get("event_total").unwrap().as_i64().unwrap();
            let endring = previous_pos - this_pos;

            //iterates over each user per gameweek events is gameweek??? ---- need to take this out od the loop will be way easier, hardcode
            // each player and have input for team ID

            by_points.insert(total, spiller);
            by_position.insert(endring, spiller);
        }
    }

    //data storage player data
    let mut overall_rank_vec: Vec<i64> = Vec::new();
    let mut in_bank_vec = Vec::new();
    let mut event_transfers_vec = Vec::new();
    let mut event_transfers_cost_vec = Vec::new();
    let mut overall_rank_vec = Vec::new();
    let mut points_gw_vec = Vec::new();
    let mut points_on_bench_vec = Vec::new();
    let mut total_points_vec = Vec::new();
    let mut team_value_vec = Vec::new();

    let player_history = reqwest::Client::new()
        .get(player_base.to_string() + team_id.to_string().as_str() + "/history/")
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&player_history).unwrap();
    let events = json.get("current").unwrap();
    for (ind, event) in events.as_array().iter().enumerate() {
        for (ind, ev) in event.iter().enumerate() {
            let in_bank = ev.get("bank").unwrap().as_f64().unwrap();
            let event_transfers = ev.get("event_transfers").unwrap().as_i64().unwrap();
            let event_transfers_cost = ev.get("event_transfers_cost").unwrap().as_i64().unwrap();
            let overall_rank = ev.get("overall_rank").unwrap().as_i64().unwrap();
            let points = ev.get("points").unwrap().as_i64().unwrap();
            let points_on_bench = ev.get("points_on_bench").unwrap().as_i64().unwrap();
            let rank = ev.get("rank").unwrap().as_i64().unwrap();
            let rank_sort = ev.get("rank_sort").unwrap().as_i64().unwrap();
            let total_points = ev.get("total_points").unwrap().as_i64().unwrap();
            let value = ev.get("value").unwrap().as_f64().unwrap();

            overall_rank_vec.push(rank);
            in_bank_vec.push(in_bank);
            event_transfers_vec.push(event_transfers);
            event_transfers_cost_vec.push(event_transfers_cost);
            overall_rank_vec.push(rank);
            points_gw_vec.push(points);
            points_on_bench_vec.push(points_on_bench);
            total_points_vec.push(total_points);
            team_value_vec.push(value);
        }
    }

    // SORTED BY POINTS WITH PLAYER
    for key in by_points.keys().sorted().rev() {
        println!("Poeng: {:?} -> {}", key, by_points[key])
    }
    // SORTED BY CHANGE IN LEAGUE POSITION WITH PLAYER
    for key in by_position.keys().sorted().rev() {
        println!("Endring: {} -> {}", key, by_position[key])
    }

    let tem_val = team_value_vec
        .iter()
        .map(|x| x / 10.0)
        .collect::<Vec<f64>>();
    // println!("{:?}", tem_val);

    Ok(())
}
