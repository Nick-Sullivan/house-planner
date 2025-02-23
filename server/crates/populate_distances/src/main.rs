use anyhow::{Context, Result};
use database::dynamodb_client_cloud::DynamoDbClient;
use database::dynamodb_client_trait::IDynamoDbClient;
use database::spatial_distance_item::SpatialDistanceItem;
use dotenv::from_path;
use geo::Polygon;
use geojson::GeoJson;
use h3o::{
    geom::{ContainmentMode, TilerBuilder},
    LatLng, Resolution,
};
use maps::{maps_client::GoogleMapsClient, models::TravelMode};
use std::{collections::HashSet, env};

#[tokio::main]
async fn main() -> Result<()> {
    let city_code = "Adelaide";
    let env_path = env::current_dir()?.join("server/.env");
    from_path(env_path).ok();
    let maps_api_key =
        env::var("GOOGLE_MAPS_API_KEY").context("GOOGLE_MAPS_API_KEY must be set")?;
    let maps_client = GoogleMapsClient::new(maps_api_key);
    let db_client = DynamoDbClient::new().await?;
    let existing_items = SpatialDistanceItem::list_from_db(city_code, &db_client)
        .await
        .context("Failed to list items")?;
    let existing_pairs: HashSet<(String, String)> = existing_items
        .into_iter()
        .map(|item| (item.source_index, item.destination_index))
        .collect();

    // Define the city's boundary in GeoJSON format, remember to repeat the first entry
    let adelaide = r#"
    {
        "type": "Polygon",
        "coordinates": [
            [
                [138.51536049346163, -34.85490824066172],
                [138.6595633990527, -34.84255603861368],
                [138.6873056569005, -34.95834949048342],
                [138.5194630253865, -34.96855376947172],
                [138.51536049346163, -34.85490824066172]
            ]
        ]
    }
    "#;
    let city_geojson = adelaide;
    let geojson: GeoJson = city_geojson.parse().unwrap();
    let polygon: Polygon<f64> = match geojson {
        GeoJson::Geometry(geometry) => geometry.value.try_into()?,
        _ => return Err(anyhow::anyhow!("Invalid GeoJSON")),
    };
    let mut tiler = TilerBuilder::new(Resolution::Seven)
        .containment_mode(ContainmentMode::Covers)
        .build();
    tiler.add(polygon)?;

    let cells = tiler.into_coverage().collect::<Vec<_>>();
    println!("Num cells: {}", cells.len());
    let num_cells = cells.len();
    for i in 0..num_cells - 1 {
        println!("Processing cell {}", i);
        let source_index = &cells[i].to_string();
        let source_lat_lng: LatLng = cells[i].into();
        for j in i + 1..num_cells {
            let destination_index = &cells[j].to_string();
            let destination_lat_lng: LatLng = cells[j].into();
            if existing_pairs.contains(&(source_index.clone(), destination_index.clone())) {
                continue;
            }
            let duration_drive = get_duration(
                maps_client
                    .get_directions(
                        source_lat_lng.lat(),
                        source_lat_lng.lng(),
                        destination_lat_lng.lat(),
                        destination_lat_lng.lng(),
                        &TravelMode::Driving,
                    )
                    .await?,
            );
            let duration_walk = get_duration(
                maps_client
                    .get_directions(
                        source_lat_lng.lat(),
                        source_lat_lng.lng(),
                        destination_lat_lng.lat(),
                        destination_lat_lng.lng(),
                        &TravelMode::Walking,
                    )
                    .await?,
            );
            let duration_cycle = get_duration(
                maps_client
                    .get_directions(
                        source_lat_lng.lat(),
                        source_lat_lng.lng(),
                        destination_lat_lng.lat(),
                        destination_lat_lng.lng(),
                        &TravelMode::Bicycling,
                    )
                    .await?,
            );
            let duration_transit = get_duration(
                maps_client
                    .get_directions(
                        source_lat_lng.lat(),
                        source_lat_lng.lng(),
                        destination_lat_lng.lat(),
                        destination_lat_lng.lng(),
                        &TravelMode::Transit,
                    )
                    .await?,
            );
            let item_forward = SpatialDistanceItem {
                city_code: city_code.to_string(),
                source_index: source_index.to_string(),
                destination_index: destination_index.to_string(),
                duration_cycle: duration_cycle,
                duration_drive: duration_drive,
                duration_transit: duration_transit,
                duration_walk: duration_walk,
            };
            let item_reverse = SpatialDistanceItem {
                city_code: city_code.to_string(),
                source_index: destination_index.to_string(),
                destination_index: source_index.to_string(),
                duration_cycle: duration_cycle,
                duration_drive: duration_drive,
                duration_transit: duration_transit,
                duration_walk: duration_walk,
            };
            db_client
                .write(vec![item_forward.save()?, item_reverse.save()?])
                .await?;
        }
    }
    Ok(())
}

fn get_duration(response: maps::models::DirectionsResponse) -> i32 {
    let mut total_duration: i32 = 0;
    for leg in &response.routes[0].legs {
        total_duration += leg.duration.value;
    }
    total_duration
}
