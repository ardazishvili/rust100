use reqwest::Error;
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    temp_max: f32,
    temp_min: f32,
    pressure: f32,
    humidity: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: u16,
}

#[derive(Deserialize, Debug)]
struct Clouds {
    all: u8,
}

#[derive(Deserialize, Debug)]
struct Sys {
    #[serde(rename = "type", default)]
    t: Option<u32>,
    #[serde(rename = "type", default)]
    id: u32,
    #[serde(rename = "type", default)]
    message: f32,
    country: String,
    sunrise: u32,
    sunset: u32,
}

#[derive(Deserialize, Debug)]
struct Message {}

#[derive(Deserialize, Debug)]
struct OkResponse {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: Main,
    #[serde(default)]
    visibility: Option<u32>,
    wind: Wind,
    clouds: Clouds,
    dt: u32,
    sys: Sys,
    timezone: i32,
    id: u32,
    name: String,
    cod: u32,
}

#[derive(Deserialize, Debug)]
struct ErrorResponse {
    cod: u32,
    message: String,
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
        StatusCode::OK => println!("200 status code : {:#?}", r.json::<OkResponse>()),
        StatusCode::UNAUTHORIZED => println!("{:#?}", r.json::<ErrorResponse>()),
        _ => panic!("No hander for this StatusCode"),
    };

    Ok(())
}
