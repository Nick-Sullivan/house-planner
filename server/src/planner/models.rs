use crate::maps::models::{Location, TravelMode};

pub struct TargetLocation {
    pub address: String,
    pub travel_mode: TravelMode,
    pub location: Location,
}
