use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HotelsRecords {
    pub id: String,
    pub city_code: String,
    pub name: String,
    pub category: f32,
    pub country_code: String,
    pub city: String,
}

pub fn get_hotels(path: String) -> Result<Vec<HotelsRecords>, Box<dyn Error>> {
    // load JSON file
    let file = File::open(path)?;
    let mut hotels = Vec::new();
    let lines = io::BufReader::new(file).lines();

    // write date to buffor
    for line in lines {
        let unit: HotelsRecords = serde_json::from_str(&line?)?;
        hotels.push(unit)
    }

    Ok(hotels)
}

#[cfg(test)]
mod test {
    use crate::{hotels::get_hotels};

    #[test]
    fn test_get_hotels() {
        get_hotels(String::from("hotels.json")).expect("test_get_hotels()");
    }

    #[test]
    #[should_panic]
    fn test_panic_get_hotels() {
        get_hotels(String::from("foo.json")).expect("test_panic_get_hotels()");
    }
}