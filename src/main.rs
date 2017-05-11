#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate csv;
#[macro_use] extern crate rocket_contrib;
extern crate rustc_serialize;

use rocket_contrib::{JSON, Value};

#[derive(RustcDecodable)]
struct RocketData {
    vehicle: String,
    origin: String,
    manufacturer: String,
    to_leo: String,
    to_gto: String,
    to_other: String,
    launches: String,
    first_flight: String,
    last_flight: String,
    retired: String
}

#[derive(Debug)]
enum ParseError {
    CSV(csv::Error)
}

impl From<csv::Error> for ParseError {
    fn from(err: csv::Error) -> ParseError {
        ParseError::CSV(err)
    }
}

fn parse_csv() -> Result<Vec<RocketData>, ParseError> {
    let mut rdr = csv::Reader::from_file("./data/data.csv")?;

    let mut rocket_data: Vec<RocketData> = Vec::new();

    for record in rdr.decode() {
        let record: RocketData = record?;

        // convert to read the headers and slugify them and then return header and string vector

        rocket_data.push(record);
    }

    Ok(rocket_data)
}

fn to_json_arr(rocket_data: Vec<RocketData>) -> Vec<Value> {
     // convert the new vector to a JSON object
     rocket_data.iter().map(|rocket| json!({
        "vehicle": rocket.vehicle,
        "origin": rocket.origin,
        "manufacturer": rocket.manufacturer,
        "to_leo": rocket.to_leo,
        "to_gto": rocket.to_gto,
        "to_other": rocket.to_other,
        "launches": rocket.launches,
        "first_fight": rocket.launches,
        "last_flight": rocket.last_flight,
        "retired": rocket.retired
     })).collect()
}

#[get("/")]
fn index() -> Result<JSON<Value>, ParseError> {
    let rocket_data = parse_csv()?;

    let rocket_json = to_json_arr(rocket_data);

    Ok(JSON(json!(rocket_json)))
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
