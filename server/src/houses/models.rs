#[derive(Clone)]
pub struct House {
    pub id: i32,
    pub address: String,
    pub url: String,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
}
