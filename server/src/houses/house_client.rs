use super::models::House;
use anyhow::Result;
use csv::ReaderBuilder;

pub struct HouseClient {
    houses: Vec<House>,
}

impl HouseClient {
    pub fn new() -> Self {
        HouseClient { houses: vec![] }
    }

    pub fn load_data(&mut self) -> Result<()> {
        let csv_data = include_str!("houses.csv");
        let mut reader = ReaderBuilder::new().from_reader(csv_data.as_bytes());
        let mut houses = Vec::new();
        let mut id = 1;
        for result in reader.records() {
            let record = result?;
            let house = House {
                id: id,
                address: record[0].to_string(),
                url: record[1].to_string(),
                lat: record[2].parse().ok(),
                lon: record[3].parse().ok(),
            };
            houses.push(house);
            id += 1;
        }
        self.houses = houses;
        Ok(())
    }

    pub fn get_houses(&self, page: usize, page_size: usize) -> Result<Vec<House>> {
        let start = (page - 1) * page_size;
        let houses = self
            .houses
            .iter()
            .skip(start)
            .take(page_size)
            .cloned()
            .collect();
        Ok(houses)
    }

    pub fn get_house_by_id(&self, id: i32) -> Result<House> {
        let house: House = self
            .houses
            .iter()
            .find(|house| house.id == id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("House not found"))?;
        Ok(house)
    }

    pub fn get_num_houses(&self) -> usize {
        self.houses.len()
    }
}
