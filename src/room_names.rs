use std::error::Error;
use std::fs::File;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RoomNamesRecords {
    pub id: String,
    pub source: String,
    pub room_name: String,
    pub room_code: String,
}

pub fn get_room_names(path: String) -> Result<Vec<RoomNamesRecords>, Box<dyn Error>> {
    // load CSV file
    let file = File::open(path)?;
    let mut room_names = Vec::new();

    // read data
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(false)
        .from_reader(file);

    // write date to buffor
    for result in rdr.deserialize::<RoomNamesRecords>() {
        room_names.push(result?);
    }

    Ok(room_names)
}

#[cfg(test)]
mod test {
    use crate::{room_names::get_room_names};

    #[test]
    fn test_get_room_names() {
        get_room_names(String::from("room_names.csv")).expect("test_get_room_names()");
    }

    #[test]
    #[should_panic]
    fn test_panic_get_room_names() {
        get_room_names(String::from("foo.csv")).expect("test_panic_get_room_names()");
    }
}