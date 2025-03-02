use anyhow::Result;
// use anyhow::{Context, Result};
// use database::dynamodb_client_cloud::DynamoDbClient;
// use database::dynamodb_client_trait::IDynamoDbClient;
// use database::spatial_distance_item::SpatialDistanceItem;
// use dotenv::from_path;
// use h3_mapper::h3_client::H3Client;
// use h3o::LatLng;
// use maps::{maps_client::GoogleMapsClient, models::TravelMode};
// use std::{collections::HashSet, env};

#[tokio::main]
async fn main() -> Result<()> {
    // let city_code = "Adelaide";
    // let env_path = env::current_dir()?.join("server/.env");
    // from_path(env_path).ok();
    // let maps_api_key =
    //     env::var("GOOGLE_MAPS_API_KEY").context("GOOGLE_MAPS_API_KEY must be set")?;
    // let maps_client = GoogleMapsClient::new(maps_api_key);
    // let db_client = DynamoDbClient::new().await?;
    // let h3_client = H3Client::new();
    // let existing_items = SpatialDistanceItem::list_from_db(city_code, &db_client)
    //     .await
    //     .context("Failed to list items")?;
    // let existing_pairs: HashSet<(String, String)> = existing_items
    //     .into_iter()
    //     .map(|item| (item.source_index, item.destination_index))
    //     .collect();
    // let cells = h3_client.get_indices_for_city(&city_code)?;
    // println!("Num cells: {}", cells.len());
    // let num_cells = cells.len();
    // for i in 0..num_cells - 1 {
    //     println!("Processing cell {}", i);
    //     let source_index = &cells[i].to_string();
    //     let source_lat_lng: LatLng = cells[i].into();
    //     for j in i + 1..num_cells {
    //         let destination_index = &cells[j].to_string();
    //         let destination_lat_lng: LatLng = cells[j].into();
    //         if existing_pairs.contains(&(source_index.clone(), destination_index.clone())) {
    //             continue;
    //         }
    //         let duration_drive = get_duration(
    //             maps_client
    //                 .get_directions(
    //                     source_lat_lng.lat(),
    //                     source_lat_lng.lng(),
    //                     destination_lat_lng.lat(),
    //                     destination_lat_lng.lng(),
    //                     &TravelMode::Driving,
    //                 )
    //                 .await?,
    //         );
    //         let duration_walk = get_duration(
    //             maps_client
    //                 .get_directions(
    //                     source_lat_lng.lat(),
    //                     source_lat_lng.lng(),
    //                     destination_lat_lng.lat(),
    //                     destination_lat_lng.lng(),
    //                     &TravelMode::Walking,
    //                 )
    //                 .await?,
    //         );
    //         let duration_cycle = get_duration(
    //             maps_client
    //                 .get_directions(
    //                     source_lat_lng.lat(),
    //                     source_lat_lng.lng(),
    //                     destination_lat_lng.lat(),
    //                     destination_lat_lng.lng(),
    //                     &TravelMode::Bicycling,
    //                 )
    //                 .await?,
    //         );
    //         let duration_transit = get_duration(
    //             maps_client
    //                 .get_directions(
    //                     source_lat_lng.lat(),
    //                     source_lat_lng.lng(),
    //                     destination_lat_lng.lat(),
    //                     destination_lat_lng.lng(),
    //                     &TravelMode::Transit,
    //                 )
    //                 .await?,
    //         );
    //         let item_forward = SpatialDistanceItem {
    //             city_code: city_code.to_string(),
    //             source_index: source_index.to_string(),
    //             destination_index: destination_index.to_string(),
    //             duration_cycle: duration_cycle,
    //             duration_drive: duration_drive,
    //             duration_transit: duration_transit,
    //             duration_walk: duration_walk,
    //         };
    //         let item_reverse = SpatialDistanceItem {
    //             city_code: city_code.to_string(),
    //             source_index: destination_index.to_string(),
    //             destination_index: source_index.to_string(),
    //             duration_cycle: duration_cycle,
    //             duration_drive: duration_drive,
    //             duration_transit: duration_transit,
    //             duration_walk: duration_walk,
    //         };
    //         db_client
    //             .write(vec![item_forward.save()?, item_reverse.save()?])
    //             .await?;
    //     }
    // }
    Ok(())
}

// fn get_duration(response: maps::models::DirectionsResponse) -> i32 {
//     let mut total_duration: i32 = 0;
//     if response.routes.is_empty() {
//         return i32::MAX;
//     }
//     for leg in &response.routes[0].legs {
//         total_duration += leg.duration.value;
//     }
//     total_duration
// }
