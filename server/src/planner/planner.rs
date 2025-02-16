use super::models::TargetLocation;
use crate::maps::maps_client::GoogleMapsClient;
use crate::{houses::models::House, maps::models::TravelMode};
use anyhow::{Context, Result};

pub struct Planner {
    maps_client: GoogleMapsClient,
    targets: Vec<TargetLocation>,
}

impl Planner {
    pub fn new(maps_client: GoogleMapsClient) -> Self {
        Planner {
            maps_client,
            targets: vec![],
        }
    }

    pub async fn add_target(&mut self, address: &str, travel_mode: TravelMode) -> Result<()> {
        let geocode_response = self.maps_client.geocode(address).await?;
        let location = geocode_response.results[0].geometry.location;
        self.targets.push(TargetLocation {
            address: address.to_string(),
            travel_mode,
            location,
        });
        Ok(())
    }

    pub async fn plan(&self, house: &House) -> Result<()> {
        let house_location = self
            .maps_client
            .geocode(&house.address)
            .await?
            .results
            .first()
            .context("No results found for the origin address.")?
            .geometry
            .location;

        let lat = house_location.lat;
        let lon = house_location.lng;
        let mut paths = vec![];
        let colors = vec![
            "0xff0000ff".to_string(),
            "0x00ff00ff".to_string(),
            "0x0000ffff".to_string(),
        ];

        for target in &self.targets {
            let directions_response = self
                .maps_client
                .get_directions(
                    lat,
                    lon,
                    target.location.lat,
                    target.location.lng,
                    &target.travel_mode,
                )
                .await?;
            let duration = &directions_response.routes[0].legs[0].duration.text;
            println!("Time to {}: {}", target.address, duration);
            let overview_polyline = &directions_response.routes[0].overview_polyline.points;
            paths.push(overview_polyline.clone());
        }

        let map_data = self
            .maps_client
            .fetch_combined_static_map_data(&paths, &colors)
            .await?;
        let file_name = format!(
            "{}.png",
            house
                .address
                .replace(" ", "_")
                .replace(",", "")
                .replace("/", "_")
        );
        self.maps_client.save_map_to_file(&map_data, &file_name)?;

        Ok(())
    }
}
