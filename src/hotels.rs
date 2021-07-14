use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use loading::Loading;

use serde::Deserialize;

//===================================================================================================================//

#[derive(Debug, Deserialize)]
pub struct HotelsRecords {
    pub id: String,
    pub city_code: String,
    pub name: String,
    pub category: f32,
    pub country_code: String,
    pub city: String,
}

//===================================================================================================================//

pub fn get_hotels(path: String) -> Result<Vec<HotelsRecords>, Box<dyn Error>> {
    // init laoding information
    let mut loading = Loading::new();
    // start laoding information
    loading.start();
    
    // load JSON file
    let file = File::open(path)?;
    let mut hotels = Vec::new();
    let lines = io::BufReader::new(file).lines();

    // write date to buffor
    for (i, line) in lines.enumerate() {
        // show loading information
        loading.text(format!("Loading hotels.json: {}", i));

        let unit: HotelsRecords = serde_json::from_str(&line.unwrap())?;
        hotels.push(unit)
    }

    // end laoding information
    loading.success("OK");
    loading.end();

    Ok(hotels)
}

//===================================================================================================================//