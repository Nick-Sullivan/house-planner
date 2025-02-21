use anyhow::{Context, Result};
use dotenv::from_path;
use houses::house_client::HouseClient;
use maps::maps_client::GoogleMapsClient;
use maps::models::TravelMode;
use planner::planner::Planner;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let env_path = env::current_dir()?.join("server/.env");
    from_path(env_path).ok();
    let maps_api_key =
        env::var("GOOGLE_MAPS_API_KEY").context("GOOGLE_MAPS_API_KEY must be set")?;
    let house_client = HouseClient::new();
    let maps_client = GoogleMapsClient::new(maps_api_key);
    let mut planner = Planner::new(maps_client);

    let houses = house_client.get_houses(1, 10)?;
    planner
        .add_target(
            "Adelaide Central Market, Adelaide SA, Australia",
            TravelMode::Driving,
        )
        .await?;
    planner
        .add_target(
            "Adelaide Airport, Adelaide SA, Australia",
            TravelMode::Driving,
        )
        .await?;

    planner.plan(&houses[0]).await?;

    Ok(())
}
