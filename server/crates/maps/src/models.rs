use serde::Deserialize;

#[derive(Deserialize)]
pub struct GeocodeResponse {
    pub results: Vec<GeocodeResult>,
}

#[derive(Deserialize)]
pub struct GeocodeResult {
    pub geometry: Geometry,
}

#[derive(Deserialize)]
pub struct Geometry {
    pub location: Location,
}

#[derive(Deserialize, Copy, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Deserialize, Debug)]
pub struct DirectionsResponse {
    pub routes: Vec<Route>,
}

#[derive(Deserialize, Debug)]
pub struct Route {
    pub overview_polyline: OverviewPolyline,
    pub legs: Vec<Leg>,
}

#[derive(Deserialize, Debug)]
pub struct OverviewPolyline {
    pub points: String,
}

#[derive(Deserialize, Debug)]
pub struct Leg {
    pub duration: Duration,
}

#[derive(Deserialize, Debug)]
pub struct Duration {
    pub text: String,
    pub value: i32,
}

pub enum TravelMode {
    Driving,
    Walking,
    Bicycling,
    Transit,
}

impl TravelMode {
    pub fn as_str(&self) -> &str {
        match self {
            TravelMode::Driving => "driving",
            TravelMode::Walking => "walking",
            TravelMode::Bicycling => "bicycling",
            TravelMode::Transit => "transit",
        }
    }
}
