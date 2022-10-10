use itertools::Itertools;
use plotters::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
// use std::io; // FOR writing for excel later
// use std::process; for writing to excel later

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let mut wtr = csv::Writer::from_writer(io::stdout()); // EXCEL WRITER FOR LATER

    //CHart builder
    let root = BitMapBackend::new("plotters-test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart_ctx = ChartBuilder::on(&root)
        .margin(10)
        .caption("Ransom data from FPL api", ("sans-serif", 40))
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(1..38, 0..150)
        .unwrap();

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

    // data storage
    let mut by_points = HashMap::new();
    let mut by_position = HashMap::new();
    let mut rank_vec = Vec::new();

    for player in players.as_array().into_iter() {
        for x in player {
            let team_id = x.get("entry").unwrap().as_i64().unwrap();
            let previous_pos = x.get("last_rank").unwrap().as_i64().unwrap();
            let this_pos = x.get("rank_sort").unwrap().as_i64().unwrap();
            let spiller = x.get("player_name").unwrap().as_str().unwrap();
            let total = x.get("event_total").unwrap().as_i64().unwrap();
            let endring = previous_pos - this_pos;

            let player_history = reqwest::Client::new()
                .get(player_base.to_string() + team_id.to_string().as_str() + "/history/")
                .send()
                .await?
                .text()
                .await?;

            let json: Value = serde_json::from_str(&player_history).unwrap();
            let events = json.get("current").unwrap();

            for event in events.as_array() {
                for ev in event {
                    let in_bank = ev.get("bank").unwrap().as_f64().unwrap();
                    let event_transfers = ev.get("event_transfers").unwrap().as_i64().unwrap();
                    let event_transfers_cost =
                        ev.get("event_transfers_cost").unwrap().as_i64().unwrap();
                    let overall_rank = ev.get("overall_rank").unwrap().as_i64().unwrap();
                    let points = ev.get("points").unwrap().as_i64().unwrap();
                    let points_on_bench = ev.get("points_on_bench").unwrap().as_i64().unwrap();
                    let rank = ev.get("rank").unwrap().as_i64().unwrap();
                    let rank_sort = ev.get("rank_sort").unwrap().as_i64().unwrap();
                    let total_points = ev.get("total_points").unwrap().as_i64().unwrap();
                    let value = ev.get("value").unwrap().as_f64().unwrap();

                    rank_vec.push(points as i32);
                    println!("^^^^{}****{}***{}", event_transfers, spiller, in_bank)
                }
            }

            // println!("{}", events);

            // println!("123{}123", player_history);

            by_points.insert(total, spiller);
            by_position.insert(endring, spiller);
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

    Ok(())
}
