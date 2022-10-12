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

    //creating excel file
    let workbook = Workbook::new("test2.xlsx");
    let format_string = workbook.add_format().set_align(FormatAlignment::Justify);
    let mut sheet2 = workbook.add_worksheet(None).unwrap();
    sheet2.write_string(0, 0, "Player", None).unwrap();
    sheet2.write_string(0, 2, "Posision cng", None).unwrap();
    sheet2.write_string(0, 4, "Points rnd", None).unwrap();

    // Using struct data to selecting and printing to excel
    let standings_root = serde_json::from_str::<data::leaguedata::Root>(&league).unwrap();
    for (i, res) in standings_root.standings.results.iter().enumerate() {
        sheet2
            .write_string(
                i as u32 + 1,
                0,
                res.player_name.as_str(),
                Some(&format_string),
            )
            .unwrap();
        sheet2
            .write_number(i as u32 + 1, 2, (res.last_rank - res.rank) as f64, None)
            .unwrap();
        sheet2
            .write_number(i as u32 + 1, 4, res.event_total as f64, None)
            .unwrap();
    }

    workbook.close().unwrap();

    Ok(())
}
