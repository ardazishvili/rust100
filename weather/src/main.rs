use crate::weather::{ErrorResponse, OkResponse};
use reqwest::Error;
use reqwest::StatusCode;

mod weather;

fn get_celsius(r: &OkResponse) -> f32 {
    r.temp() - 273.15
}

fn show_error_description(r: &ErrorResponse) -> &String {
    r.message()
}

fn main() -> Result<(), Error> {
    let request_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={city_name}&appid={id}",
        city_name = "Saratov",
        id = "ID_GOES_HERE"
    );
    println!("{}", request_url);

    let r = match reqwest::blocking::get(&request_url) {
        Ok(response) => response,
        Err(_) => panic!("Error at blocking call"),
    };
    match r.status() {
        StatusCode::OK => println!(
            "The current temperature is: {}",
            get_celsius(&r.json::<OkResponse>()?)
        ),
        StatusCode::UNAUTHORIZED => println!(
            "The error is: {}",
            show_error_description(&r.json::<ErrorResponse>()?)
        ),
        _ => panic!("No hander for this StatusCode"),
    };

    Ok(())
}
