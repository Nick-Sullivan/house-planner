use super::models::{DirectionsResponse, GeocodeResponse, TravelMode};
use anyhow::Result;
use reqwest::Client;
use std::fs::File;
use std::io::{copy, Cursor};

pub struct GoogleMapsClient {
    api_key: String,
    client: Client,
}

impl GoogleMapsClient {
    pub fn new(api_key: String) -> Self {
        GoogleMapsClient {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn geocode(&self, address: &str) -> Result<GeocodeResponse> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/geocode/json?address={}&key={}",
            address, self.api_key
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<GeocodeResponse>()
            .await?;
        Ok(response)
    }

    pub async fn get_directions(
        &self,
        origin_lat: f64,
        origin_lng: f64,
        destination_lat: f64,
        destination_lng: f64,
        travel_mode: &TravelMode,
    ) -> Result<DirectionsResponse> {
        let url = format!(
            "https://maps.googleapis.com/maps/api/directions/json?origin={},{}&destination={},{}&mode={}&key={}",
            origin_lat, origin_lng, destination_lat, destination_lng, travel_mode.as_str(), self.api_key
        );
        let response = self
            .client
            .get(&url)
            .send()
            .await?
            .json::<DirectionsResponse>()
            .await?;
        Ok(response)
    }

    pub async fn fetch_combined_static_map_data(
        &self,
        paths: &[String],
        colors: &[String],
    ) -> Result<Vec<u8>> {
        let path_param: String = paths
            .iter()
            .zip(colors.iter())
            .map(|(path, color)| format!("path=weight:5|color:{}|enc:{}", color, path))
            .collect::<Vec<String>>()
            .join("&");
        let map_url = format!(
            "https://maps.googleapis.com/maps/api/staticmap?size=600x300&{}&key={}",
            path_param, self.api_key
        );
        let response = self.client.get(map_url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub fn save_map_to_file(&self, data: &[u8], file_path: &str) -> Result<()> {
        let mut file = File::create(file_path)?;
        let mut cursor = Cursor::new(data);
        copy(&mut cursor, &mut file)?;

        println!("Map image saved as {}", file_path);

        Ok(())
    }
}
