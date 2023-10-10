use serde::{ Deserialize, Serialize };
use serde_json::json;
use reqwest::get;
use sqlx::{Pool, Postgres};
use askama::Template;

use crate::error::{ Result, Error };

#[derive(sqlx::FromRow, Deserialize, Serialize, Debug, Clone)]
pub struct GeoResponse {
    pub name: String,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Template, Deserialize, Serialize, Debug, Clone)]
#[template(path = "weather.html")]
pub struct WeatherRespose {
    pub main: Main
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Main {
    pub temp: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub temp_min: f64,
    pub temp_max: f64,
}

async fn fetch_coords(city: &String) -> Result<GeoResponse> {
    let endpoint = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={}&limit={}&appid={}", 
        city, 
        "1", 
        "94f2b6872c062c489c8f0e75a03ccc5a"
    );
    let res: Vec<GeoResponse> = get(&endpoint).await?.json().await?;
    Ok(res
        .get(0)
        .cloned()
        .unwrap()
    )
}

pub async fn get_coords(city: &String, pool: &Pool<Postgres>) -> Result<GeoResponse> {
    let coords = sqlx::query_as::<_, GeoResponse>(
        "SELECT name, latitude AS lat, longitude AS lon FROM cities WHERE name = $1"
    )
    .bind(city)
    .fetch_optional(pool)
    .await?;

    if let Some(coords) = coords {
        println!("->> {:<12} Select", "DATABASE");
        return Ok(coords);
    }

    let coords = fetch_coords(city).await?;
    println!("->> {:<12} Insert", "DATABASE");
    sqlx::query("INSERT INTO cities (name, latitude, longitude) VALUES ($1, $2, $3)")
        .bind(city)
        .bind(coords.lat)
        .bind(coords.lon)
        .execute(pool)
        .await?;

    Ok(coords)
}

pub async fn fetch_weather(city: &String, pool: &Pool<Postgres>) -> Result<Main> {
    let coords = get_coords(city, pool).await?;
    let endpoint = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid=94f2b6872c062c489c8f0e75a03ccc5a",
        coords.lat,
        coords.lon,
    );
    let res: WeatherRespose = get(&endpoint).await?.json().await?;
    Ok(res.main.clone())
}
