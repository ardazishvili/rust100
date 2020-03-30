use crate::weather::{ErrorResponse, OkResponse};
use reqwest::Error;
use reqwest::StatusCode;
use std::str::FromStr;
use structopt::StructOpt;

mod weather;

enum Param {
    Temp,
    Humidity,
    Wind,
}

impl FromStr for Param {
    type Err = std::string::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = match s {
            "t" => Param::Temp,
            "humidity" => Param::Humidity,
            "wind" => Param::Wind,
            _ => panic!("Wrong parameters"),
        };

        Ok(res)
    }
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(short = "c", long = "city")]
    city: String,
    #[structopt(short = "p", long = "parameter")]
    param: Param,
}

fn get_celsius(r: &OkResponse) -> f32 {
    r.temp() - 273.15
}

fn show_error_description(r: &ErrorResponse) -> &String {
    r.message()
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();

    let request_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={city_name}&appid={id}",
        city_name = args.city,
        id = "92a3fc2f269949f3f7c3e7871ddafe7f"
    );

    let r = match reqwest::blocking::get(&request_url) {
        Ok(response) => response,
        Err(_) => panic!("Error at blocking call"),
    };
    match r.status() {
        StatusCode::OK => match args.param {
            Param::Temp => println!(
                "The current temperature is: {}",
                get_celsius(&r.json::<OkResponse>()?)
            ),
            _ => println!("Parameter to be added..."),
        },
        StatusCode::UNAUTHORIZED => println!(
            "The error is: {}",
            show_error_description(&r.json::<ErrorResponse>()?)
        ),
        StatusCode::NOT_FOUND => println!("No data for this city: {}", r.status()),
        _ => panic!("No hander for this status: {}", r.status()),
    };

    Ok(())
}
