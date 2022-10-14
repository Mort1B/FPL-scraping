use super::playerdata;
use core::ops::Range;
use plotters::prelude::*;
use xlsxwriter::*;

pub async fn get_player_data() -> Result<(), String> {
    // xlsxwriter, printing .png chart instead of charing in excel
    let workbook = Workbook::new("test1.xlsx");
    let format1 = workbook.add_format().set_font_color(FormatColor::Blue);
    let mut sheet1 = workbook.add_worksheet(None).unwrap();
    sheet1
        .write_string(0, 0, "Points on Bench", Some(&format1))
        .unwrap();

    // Base URL for player data
    let team_id = 657266;
    let player_base = "https://fantasy.premierleague.com/api/entry/";

    // Use reqwest to get API data
    let player_history = reqwest::Client::new()
        .get(player_base.to_string() + team_id.to_string().as_str() + "/history/")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Data storage
    let mut pointsvec = Vec::new();

    // Using Deserialized structs to select data from API and loop through
    //to set data and write to excel file
    let player_root = serde_json::from_str::<playerdata::Root>(&player_history).unwrap();
    for (i, curr) in player_root.current.iter().enumerate() {
        pointsvec.push(curr.points_on_bench as i32);
        sheet1
            .write_number(
                i as u32 + 1,
                0,
                curr.points_on_bench.as_f64(),
                Some(&format1),
            )
            .unwrap();
    }

    draw_chart("image.png", 0..40, 0..50, pointsvec).unwrap();

    // inserting plotters image in excel and writing the .xlsx file
    sheet1.insert_image(2, 2, "plotters-net.png").unwrap();
    workbook.close().unwrap();

    Ok(())
}

fn draw_chart(
    path: &str,
    x_spec: Range<i32>,
    y_spec: Range<i32>,
    data: Vec<i32>,
) -> Result<(), String> {
    //Chart setup, printing image
    let root = BitMapBackend::new(path, (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Points on bench", ("sans-serif", 40))
        .build_cartesian_2d(x_spec, y_spec)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    //Drawing to the png
    ctx.draw_series(LineSeries::new(
        (0..).zip(data[..].iter()).map(|(idx, y)| (idx, *y)),
        &BLUE,
    ))
    .unwrap();
    root.present().unwrap();

    Ok(())
}
