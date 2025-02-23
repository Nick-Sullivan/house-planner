use anyhow::{Error, Result};
use geo::Polygon;
use geojson::GeoJson;
use h3o::{
    geom::{ContainmentMode, TilerBuilder},
    CellIndex, Resolution,
};

pub struct H3Client {}

impl H3Client {
    pub fn new() -> Self {
        H3Client {}
    }

    pub fn get_indices_for_city(&self, _city_code: &str) -> Result<Vec<CellIndex>, Error> {
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
        Ok(cells)
    }
}
