mod league;
mod player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    league::league::get_league_data().await.unwrap();
    player::player::get_player_data().await.unwrap();
    Ok(())
}
