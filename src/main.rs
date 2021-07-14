use crate::command_line::Config;
use crate::hotels::HotelsRecords;
use crate::room_names::RoomNamesRecords;

use std::error::Error;
use std::fs::File;
use std::{env, ops::Add};
use std::{process, thread};

use chrono;
use loading::Loading;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};

mod command_line;
mod hotels;
mod room_names;
mod test;

//===================================================================================================================//

#[derive(Debug, Deserialize)]
struct InputRecords {
    city_code: String,
    hotel_code: String,
    room_type: String,
    room_code: String,
    meal: String,
    checkin: String,
    adults: u32,
    children: u32,
    price: f64,
    source: String,
}

//===================================================================================================================//

#[derive(Debug, Serialize)]
struct OutputRecords {
    #[serde(rename = "room_type meal")]
    room_type_meal: String,
    room_code: String,
    source: String,
    hotel_name: String,
    city_name: String,
    city_code: String,
    hotel_category: f32,
    pax: u64,
    adults: u32,
    children: u32,
    room_name: String,
    checkin: String,
    checkout: String,
    price: String,
}

//===================================================================================================================//

// convert an input file using supplementary files to an output file
fn input_from_to_output(config: Config) -> Result<(), Box<dyn Error>> {
    // init laoding information
    let mut loading = Loading::new();
    // start laoding information
    loading.start();
    // read input file path
    let file = File::open(config.input_filename)?;

    // varibale with file reader
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'|')
        .has_headers(true)
        .from_reader(file);

    // get supplementary files
    let (hotels, room_names) =
        get_supplementary_files(String::from("hotels.json"), String::from("room_names.csv"))?;

    // create a file to save
    let file = File::create(config.output_filename)?;
    let mut wtr = WriterBuilder::new().delimiter(b';').from_writer(file);

    // reading input file
    for (i, result) in rdr.deserialize::<InputRecords>().enumerate() {
        // read record
        let mut record = result?;
        // correct date
        correct_date(&mut record.checkin);
        // Set the check-out date as the check-in date plus one to the check-in date
        let checkout = add_days(&record.checkin, 1)?;
        // marge two records in one
        let room_type_meal = record.room_type.add(" ").add(&record.meal);
        // calculate pax
        let pax = calculate_pax(record.adults, record.children);
        // calculate price
        let price = format!("{:.2}", (record.price / pax as f64));

        // read hotel data from hotels data names
        for hotel in &hotels {
            if &hotel.id == &record.hotel_code {
                for name in &room_names {
                    if &name.room_code == &record.room_code
                        && &name.source == &record.source
                        && record.hotel_code == name.id
                    {
                        // add data to buffer
                        wtr.serialize(OutputRecords {
                            room_type_meal: room_type_meal.to_owned(),
                            room_code: record.room_code.to_owned(),
                            source: record.source.to_owned(),
                            hotel_name: hotel.name.to_owned(),
                            city_name: hotel.city.to_owned(),
                            city_code: record.city_code.to_owned(),
                            hotel_category: hotel.category,
                            pax,
                            adults: record.adults,
                            children: record.children,
                            room_name: name.room_name.to_owned(),
                            checkin: record.checkin.to_owned(),
                            checkout: checkout.to_owned(),
                            price: price.to_owned(),
                        })?;
                    }
                }
            }
        }
        // show loading information
        loading.text(format!("Create output.csv : {}", i));
    }

    // write data to new file
    wtr.flush()?;

    // end laoding information
    loading.success("OK");
    loading.end();

    Ok(())
}

//===================================================================================================================//

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = input_from_to_output(config) {
        println!("{}", err);
        process::exit(1);
    }
}

//===================================================================================================================//

// change date to a readable form
fn correct_date(data: &mut String) {
    data.insert(4, '-');
    data.insert(7, '-');
}

//===================================================================================================================//

fn get_supplementary_files(
    hotels_path: String,
    room_names_path: String,
) -> Result<(Vec<HotelsRecords>, Vec<RoomNamesRecords>), Box<dyn Error>> {
    let hotels_handle = thread::spawn(|| {
        let this = hotels::get_hotels(hotels_path);
        match this {
            Ok(t) => t,
            Err(e) => panic!("hotels.json invalid data {}", &e),
        }
    });

    let room_names_handle = thread::spawn(|| {
        let this = room_names::get_room_names(room_names_path);
        match this {
            Ok(t) => t,
            Err(e) => panic!("room_names.csv {}", &e),
        }
    });

    let hotels = hotels_handle.join().expect("hotels_handle error");
    let room_names = room_names_handle.join().expect("room_names_handle error");

    Ok((hotels, room_names))
}

//===================================================================================================================//

fn calculate_pax(adults: u32, children: u32) -> u64 {
    let adults = adults as u64;
    let children = children as u64;

    let this = adults.checked_add(children);
    match this {
        Some(val) => val as u64,
        None => panic!("add error, validate data"),
    }
}

//===================================================================================================================//

fn add_days(checkin: &String, days: i64) -> Result<String, Box<dyn Error>> {
    let checkout = chrono::NaiveDate::parse_from_str(checkin, "%Y-%m-%d")?
        .add(chrono::Duration::days(days))
        .to_string();

    Ok(checkout)
}

//===================================================================================================================//