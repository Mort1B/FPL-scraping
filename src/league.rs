use xlsxwriter::*;

use crate::data;

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

    // data storage league data
    // let mut by_points = HashMap::new();
    // let mut by_position = HashMap::new();

    let standings_root = serde_json::from_str::<data::leaguedata::Standings>(&league).unwrap();

    let mut news = Vec::new();

    for (i, res) in standings_root.results.iter().enumerate() {
        println!("{}", res.player_name);
        news.push(res.player_name.as_str())
    }

    let workbook = Workbook::new("test2.xlsx");
    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1.write_string(0, 1, &news[0], None).unwrap();
    sheet1.write_string(1, 1, &news[1], None).unwrap();
    sheet1.write_string(2, 1, &news[2], None).unwrap();
    sheet1.write_string(3, 1, &news[3], None).unwrap();

    Ok(())
}
