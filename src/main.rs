use crate::command_line::Config;
use structopt::StructOpt;
use crate::hotels::HotelsRecords;
use crate::room_names::RoomNamesRecords;

use std::error::Error;
use std::fs::File;
use std::ops::Add;
use std::{process, thread};

use chrono;
use csv::WriterBuilder;
use serde::{Deserialize, Serialize};

mod command_line;
mod hotels;
mod room_names;

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

#[derive(Debug, Serialize)]
struct OutputRecords<'a> {
    #[serde(rename = "room_type meal")]
    room_type_meal: &'a str,
    room_code: &'a str,
    source: &'a str,
    hotel_name: &'a str,
    city_name: &'a str,
    city_code: &'a str,
    hotel_category: f32,
    pax: u64,
    adults: u32,
    children: u32,
    room_name: &'a str,
    checkin: &'a str,
    checkout: &'a str,
    price: &'a str,
}

// convert an input file using supplementary files to an output file
fn input_from_to_output(config: Config) -> Result<(), Box<dyn Error>> {
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
    let output_filename= match config.output_filename {
        Some(output) => output,
        None => String::from("output.csv"),
    };

    let file = File::create(output_filename)?;
    let mut wtr = WriterBuilder::new().delimiter(b';').from_writer(file);

    // reading input file
    for result in rdr.deserialize::<InputRecords>() {
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

        // auxiliary buffers
        let mut hotel_name = "".to_string();
        let mut city_name= "".to_string();
        let mut room_name= "".to_string();
        let mut hotel_category= 0.0;

        for hotel in &hotels {            
            if &hotel.id == &record.hotel_code {
                hotel_name = hotel.name.clone();
                city_name = hotel.city.clone();
                hotel_category = hotel.category;
                break;
            }
        }
        
        for name in &room_names {
            if &name.room_code == &record.room_code
                && &name.source == &record.source
                && record.hotel_code == name.id
            {
                room_name = name.room_name.clone();
                break;
            }
        }

        wtr.serialize(OutputRecords {
            room_type_meal: &room_type_meal.to_owned(),
            room_code: &record.room_code.to_owned(),
            source: &record.source.to_owned(),
            hotel_name: &hotel_name, 
            city_name: &city_name.to_owned(),
            city_code: &record.city_code.to_owned(),
            hotel_category: hotel_category.to_owned(),
            pax,
            adults: record.adults,
            children: record.children,
            room_name: &room_name.to_owned(),
            checkin: &record.checkin.to_owned(),
            checkout: &checkout.to_owned(),
            price: &price.to_owned(),
        })?;
    }

    // write data to new file
    wtr.flush()?;

    Ok(())
}

fn main() {
    let args = Config::from_args();

    if let Err(err) = input_from_to_output(args) {
        println!("{}", err);
        process::exit(1);
    }
}

// change date to a readable form
fn correct_date(data: &mut String) {
    data.insert(4, '-');
    data.insert(7, '-');
}

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

fn calculate_pax(adults: u32, children: u32) -> u64 {
    let adults = adults as u64;
    let children = children as u64;

    let this = adults.checked_add(children);
    match this {
        Some(val) => val as u64,
        None => panic!("add error, validate data"),
    }
}

fn add_days(checkin: &String, days: i64) -> Result<String, Box<dyn Error>> {
    let checkout = chrono::NaiveDate::parse_from_str(checkin, "%Y-%m-%d")?
        .add(chrono::Duration::days(days))
        .to_string();

    Ok(checkout)
}

#[cfg(test)]
mod test {
    use crate::{add_days, calculate_pax, correct_date, get_supplementary_files};

    #[test]
    fn test_get_supplementary_files() {
        get_supplementary_files(String::from("hotels.json"), String::from("room_names.csv"))
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn test_panic_get_supplementary_files() {
        get_supplementary_files(String::from("sletoh.json"), String::from("seman_moor.csv"))
            .unwrap();
    }

    #[test]
    fn test_correct_date() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);
    }

    #[test]
    fn test_add_days() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);

        let day = add_days(&date, 1).unwrap();
        assert!(String::from("2001-09-15") == day);
    }

    #[test]
    #[should_panic]
    fn test_panic_add_days() {
        let mut date = String::from("20010914");
        correct_date(&mut date);

        assert!(String::from("20010914") != date);
        assert!(String::from("2001-09-14") == date);

        let day = add_days(&date, 1).unwrap();
        assert!(String::from("2001-09-15") == day);
        assert!(String::from("20010915") == day);
    }

    #[test]
    fn test_calculate_pax() {
        let pax = calculate_pax(20, 20);
        assert!(pax == 40);

        let pax = calculate_pax(std::u32::MAX, std::u32::MAX);
        assert!(pax == 8589934590);
    }
}