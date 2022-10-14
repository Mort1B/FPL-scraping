use super::playerdata;
use core::ops::Range;
use plotters::prelude::*;
use std::error::Error;
use xlsxwriter::*;

// FIX writing to excel function -> Fix league -> Get everything into same excel -> refactor
pub async fn get_player_data() -> Result<(), Box<dyn Error + Send + Sync>> {
    // xlsxwriter, printing .png chart instead of charing in excel
    let workbook = Workbook::new("test1.xlsx");
    let mut sheet1 = workbook.add_worksheet(None)?;
    let mut sheet2 = workbook.add_worksheet(None)?;
    let mut sheet3 = workbook.add_worksheet(None)?;
    let mut sheet4 = workbook.add_worksheet(None)?;
    let mut sheet5 = workbook.add_worksheet(None)?;
    let mut sheet6 = workbook.add_worksheet(None)?;
    let mut sheet7 = workbook.add_worksheet(None)?;
    let mut sheet8 = workbook.add_worksheet(None)?;
    sheet1.write_string(0, 0, "points_vec", None)?;
    sheet2.write_string(0, 0, "rank_vec", None)?;
    sheet3.write_string(0, 0, "overall_rank_vec", None)?;
    sheet4.write_string(0, 0, "bank_vec", None)?;
    sheet5.write_string(0, 0, "team_value_vec", None)?;
    sheet6.write_string(0, 0, "event_transfers_vec", None)?;
    sheet7.write_string(0, 0, "event_transfer_cost_vec", None)?;
    sheet8.write_string(0, 0, "Points on Bench", None)?;

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
    let mut points_vec = Vec::new();
    let mut rank_vec = Vec::new();
    let mut overall_rank_vec = Vec::new();
    let mut bank_vec = Vec::new();
    let mut team_value_vec = Vec::new();
    let mut event_transfers_vec = Vec::new();
    let mut event_transfer_cost_vec = Vec::new();
    let mut points_on_bench_vec = Vec::new();

    // Using Deserialized structs to select data from API and loop through
    //to set data and write to excel file
    let player_root = serde_json::from_str::<playerdata::Root>(&player_history)?;
    for (i, curr) in player_root.current.iter().enumerate() {
        points_vec.push(curr.points as i32);
        rank_vec.push(curr.rank as i32);
        overall_rank_vec.push(curr.overall_rank as i32);
        bank_vec.push(curr.bank as i32);
        team_value_vec.push(curr.value as i32);
        event_transfers_vec.push(curr.event_transfers as i32);
        event_transfer_cost_vec.push(curr.event_transfers_cost as i32);
        points_on_bench_vec.push(curr.points_on_bench as i32);

        sheet1.write_number(i as u32 + 1, 0, curr.points.as_f64(), None)?;
        sheet2.write_number(i as u32 + 1, 0, curr.rank.as_f64(), None)?;
        sheet3.write_number(i as u32 + 1, 0, curr.overall_rank.as_f64(), None)?;
        sheet4.write_number(i as u32 + 1, 0, curr.bank.as_f64(), None)?;
        sheet5.write_number(i as u32 + 1, 0, curr.value.as_f64(), None)?;
        sheet6.write_number(i as u32 + 1, 0, curr.event_transfers.as_f64(), None)?;
        sheet7.write_number(i as u32 + 1, 0, curr.event_transfers_cost.as_f64(), None)?;
        sheet8.write_number(i as u32 + 1, 0, curr.points_on_bench.as_f64(), None)?;
    }

    // DRAWING CHARTS IN THIS REGION

    draw_chart("points.png", "points", 0..40, 0..150, points_vec)?;
    draw_chart("gw_rank.png", "GW rank", 0..40, 0..10000000, rank_vec)?;
    draw_chart(
        "overall_rank.png",
        "overall rank",
        0..40,
        0..8000000,
        overall_rank_vec,
    )?;
    draw_chart("bank.png", "bank", 0..40, 0..50, bank_vec)?;
    draw_chart(
        "team_value.png",
        "team_value",
        0..40,
        900..1100,
        team_value_vec,
    )?;
    draw_chart(
        "event_transfers.png",
        "event_transfers",
        0..40,
        0..10,
        event_transfers_vec,
    )?;
    draw_chart(
        "transfer_cost.png",
        "transfer cost",
        0..40,
        0..20,
        event_transfer_cost_vec,
    )?;
    draw_chart(
        "points_on_bench.png",
        "points on bench",
        0..40,
        0..25,
        points_on_bench_vec,
    )?;

    // inserting plotters image in excel and writing the .xlsx file
    sheet1.insert_image(2, 2, "points.png")?;
    sheet2.insert_image(2, 2, "gw_rank.png")?;
    sheet3.insert_image(2, 2, "overall_rank.png")?;
    sheet4.insert_image(2, 2, "bank.png")?;
    sheet5.insert_image(2, 2, "team_value.png")?;
    sheet6.insert_image(2, 2, "event_transfers.png")?;
    sheet7.insert_image(2, 2, "transfer_cost.png")?;
    sheet8.insert_image(2, 2, "points_on_bench.png")?;
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
