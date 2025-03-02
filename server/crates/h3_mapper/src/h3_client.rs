use anyhow::{Error, Result};
use csv::ReaderBuilder;
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
    pub fn get_indices_for_city(&self, _city_code: &str) -> Result<Vec<String>, Error> {
        let csv_data = include_str!("adelaide_boundary.csv");
        let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
        let mut items = Vec::new();
        for result in reader.records() {
            let record = result?;
            let h3_index = record[0].to_string();
            items.push(h3_index);
        }
        Ok(items)
    }

    pub fn get_cell_indices_for_city(&self, _city_code: &str) -> Result<Vec<CellIndex>, Error> {
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
        // let adelaide = r#"
        // {
        //     "type": "Polygon",
        //     "coordinates": [
        //         [
        //             [138.3788372027332, -34.55471765878935],
        //             [138.98283859854226, -34.54192062482642],
        //             [139.0177968795236, -35.31258726750041],
        //             [138.40020059699214, -35.306247834898016],
        //             [138.3788372027332, -34.55471765878935]
        //         ]
        //     ]
        // }
        // "#;

        let city_geojson = adelaide;
        let geojson: GeoJson = city_geojson.parse().unwrap();
        let polygon: Polygon<f64> = match geojson {
            GeoJson::Geometry(geometry) => geometry.value.try_into()?,
            _ => return Err(anyhow::anyhow!("Invalid GeoJSON")),
        };
        let mut tiler = TilerBuilder::new(Resolution::Eight)
            .containment_mode(ContainmentMode::Covers)
            .build();
        tiler.add(polygon)?;
        let cells = tiler.into_coverage().collect::<Vec<_>>();
        Ok(cells)
    }
}
