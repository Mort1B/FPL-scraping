use plotters::prelude::*;
use serde::{Deserialize, Serialize};

pub async fn get_league_data() -> Result<(), String> {
    // Getting League info from API
      let league = reqwest::Client::new()
          .get("https://fantasy.premierleague.com/api/leagues-classic/41686/standings/")
          .send()
          .await
          .unwrap()
          .text()
          .await
          .unwrap();
  
      let json: Value = serde_json::from_str(&league).unwrap();
      let players = json.get("standings").unwrap().get("results").unwrap();
  
  
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


          // SORTED BY POINTS WITH PLAYER
    for key in by_points.keys().sorted().rev() {
        println!("Poeng: {:?} -> {}", key, by_points[key])
    }
    // SORTED BY CHANGE IN LEAGUE POSITION WITH PLAYER
    for key in by_position.keys().sorted().rev() {
        println!("Endring: {} -> {}", key, by_position[key])
    }

} 