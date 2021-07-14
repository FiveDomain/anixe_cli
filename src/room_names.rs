use std::error::Error;
use std::fs::File;

use loading::Loading;
use serde::Deserialize;

//===================================================================================================================//

#[derive(Debug, Deserialize)]
pub struct RoomNamesRecords {
    pub id: String,
    pub source: String,
    pub room_name: String,
    pub room_code: String,
}

//===================================================================================================================//

pub fn get_room_names(path: String) -> Result<Vec<RoomNamesRecords>, Box<dyn Error>> {
    // init laoding information
    let mut loading = Loading::new();
    // start laoding information
    loading.start();

    // load CSV file
    let file = File::open(path)?;
    let mut room_names = Vec::new();

    // read data
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    // write date to buffor
    for (i, result) in rdr.deserialize::<RoomNamesRecords>().enumerate() {
        room_names.push(result?);

        // show loading information
        loading.text(format!("Loading room_names.csv: {}", i));
    }

    // end laoding information
    loading.success("OK");
    loading.end();

    Ok(room_names)
}

//===================================================================================================================//