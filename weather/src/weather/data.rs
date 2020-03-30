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
pub struct OkResponse {
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
pub struct ErrorResponse {
    cod: u32,
    message: String,
}

impl OkResponse {
    pub fn temp(&self) -> f32 {
        self.main.temp
    }
}

impl ErrorResponse {
    pub fn message(&self) -> &String {
        &self.message
    }
}
