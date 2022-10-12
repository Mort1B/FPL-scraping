use plotters::prelude::*;
use xlsxwriter::*;

mod data;
mod league;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //attempt 100 at chart
    let root = BitMapBackend::new("plotters-net.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Points on bench", ("sans-serif", 40))
        .build_cartesian_2d(0..40, 0..30)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    // xlsxwriter testing, cannot get it to write chart to file
    let workbook = Workbook::new("test1.xlsx");
    let format1 = workbook.add_format().set_font_color(FormatColor::Blue);
    let mut sheet1 = workbook.add_worksheet(None).unwrap();

    // Base URL for player data
    let team_id = 657266;
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

    let player_root = serde_json::from_str::<data::playerdata::Root>(&player_history).unwrap();
    for (i, curr) in player_root.current.iter().enumerate() {
        pointsvec.push(curr.points_on_bench as i32);
        sheet1
            .write_string(
                i as u32,
                0,
                curr.points_on_bench.to_string().as_str(),
                Some(&format1),
            )
            .unwrap();
    }

    ctx.draw_series(LineSeries::new(
        (0..).zip(pointsvec[..].iter()).map(|(idx, y)| (idx, *y)),
        &BLUE,
    ))
    .unwrap();
    root.present().unwrap();

    sheet1.insert_image(2, 2, "plotters-net.png").unwrap();
    workbook.close().unwrap();

    Ok(())
}
