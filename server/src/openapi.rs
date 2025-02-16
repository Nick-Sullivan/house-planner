#![allow(dead_code)]
mod endpoints;
mod houses;

use std::fs::File;
use std::io::Write;

fn main() {
    let (_, api) = endpoints::router::create_router().split_for_parts();
    let openapi_json = serde_json::to_string_pretty(&api).unwrap();
    let mut file = File::create("openapi.json").unwrap();
    file.write_all(openapi_json.as_bytes()).unwrap();
}
