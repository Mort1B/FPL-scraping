use super::playerdata;
use core::ops::Range;
use plotters::prelude::*;
use std::error::Error;
use xlsxwriter::*;

// FIX writing to excel function -> Fix league -> Get everything into same excel -> refactor
// Get data for all players and print graphs for everyone on each page -> insert ID's from all players or take them from league? then loop over the program with all ID's
pub async fn get_player_data() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Creating workbook and sheets with strings
    let workbook = Workbook::new("player_data.xlsx");
    let mut sheets = Vec::new();
    let sheet_names = [
        "points_vec",
        "rank_vec",
        "overall_rank_vec",
        "bank_vec",
        "team_value_vec",
        "event_transfers_vec",
        "event_transfer_cost_vec",
        "Points on Bench",
    ];
    for name in &sheet_names {
        sheets.push(workbook.add_worksheet(Some(name))?);
    }

    // Base URL for player data
    let team_id = 657266;
    let player_base = "https://fantasy.premierleague.com/api/entry/";

    // Use reqwest to get API data
    let player_history = reqwest::Client::new()
        .get(player_base.to_string() + team_id.to_string().as_str() + "/history/")
        .send()
        .await?
        .text()
        .await?;

    // Data storage
    let mut data = Vec::new();
    for _ in 0..sheet_names.len() {
        data.push(Vec::new());
    }

    // Using Deserialized structs to select data from API and loop through
    //to set data and write to excel file
    let player_root = serde_json::from_str::<playerdata::Root>(&player_history)?;
    for (i, curr) in player_root.current.iter().enumerate() {
        data[0].push(curr.points as i32);
        data[1].push(curr.rank as i32);
        data[2].push(curr.overall_rank as i32);
        data[3].push(curr.bank as i32);
        data[4].push(curr.value as i32);
        data[5].push(curr.event_transfers as i32);
        data[6].push(curr.event_transfers_cost as i32);
        data[7].push(curr.points_on_bench as i32);

        for (j, sheet) in sheets.iter_mut().enumerate() {
            sheet.write_number(i as u32 + 1, 0, data[j][i].into(), None)?;
        }
    }

    // DRAWING CHARTS IN THIS REGION

    draw_chart("points.png", "points", 0..40, 0..150, data[0].clone())?;
    draw_chart(
        "gw_rank.png",
        "GW rank",
        0..40,
        0..10000000,
        data[1].clone(),
    )?;
    draw_chart(
        "overall_rank.png",
        "overall rank",
        0..40,
        0..8000000,
        data[2].clone(),
    )?;
    draw_chart("bank.png", "bank", 0..40, 0..50, data[3].clone())?;
    draw_chart(
        "team_value.png",
        "team_value",
        0..40,
        900..1100,
        data[4].clone(),
    )?;
    draw_chart(
        "event_transfers.png",
        "event_transfers",
        0..40,
        0..10,
        data[5].clone(),
    )?;
    draw_chart(
        "transfer_cost.png",
        "transfer cost",
        0..40,
        0..20,
        data[6].clone(),
    )?;
    draw_chart(
        "points_on_bench.png",
        "points on bench",
        0..40,
        0..25,
        data[7].clone(),
    )?;

    // inserting plotters image in excel and writing the .xlsx file
    sheets[0].insert_image(0, 1, "points.png")?;
    sheets[1].insert_image(0, 1, "gw_rank.png")?;
    sheets[2].insert_image(0, 1, "overall_rank.png")?;
    sheets[3].insert_image(0, 1, "bank.png")?;
    sheets[4].insert_image(0, 1, "team_value.png")?;
    sheets[5].insert_image(0, 1, "event_transfers.png")?;
    sheets[6].insert_image(0, 1, "transfer_cost.png")?;
    sheets[7].insert_image(0, 1, "points_on_bench.png")?;

    workbook.close()?;

    Ok(())
}

fn draw_chart(
    path: &str,
    caption: &str,
    x_spec: Range<i32>,
    y_spec: Range<i32>,
    data: Vec<i32>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    //Chart setup, printing image
    let root = BitMapBackend::new(path, (700, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(caption, ("sans-serif", 40))
        .build_cartesian_2d(x_spec, y_spec)?;

    ctx.configure_mesh().draw()?;

    //Drawing to the png
    ctx.draw_series(LineSeries::new(
        (0..).zip(data[..].iter()).map(|(idx, y)| (idx, *y)),
        &BLUE,
    ))?;

    root.present()?;

    Ok(())
}

// HOW TO DO THIS???
fn insert_into_excel(workbook: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    //return vec of sheets and insert into each sheet?
    // let mut sheets =
    // for i in 0..=9 {
    //     // sheetname = i
    // }
    Ok(())
}
