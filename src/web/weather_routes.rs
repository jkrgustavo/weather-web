use axum::{ Router, routing::get, response::{IntoResponse, Html}, extract::{Path, State}, Json };
use serde::Deserialize;
use sqlx::{Pool, Postgres}; 
use crate::error::{ Error, Result };

use crate::api::*;

pub fn weather_routes(pool: Pool<Postgres>) -> Router {
    Router::new()
    .route("/weather/:city", get(weather_handler))
    //.route("/coords/:city", get(coord_handler))
    .with_state(pool)
}

async fn coord_handler(
    Path(city): Path<String>,
    State(pool): State<Pool<Postgres>>
) -> Result<Json<GeoResponse>> {
    println!("->> {:<12} Coordinates Handler", "HANDLER");

    let coord = get_coords(&city, &pool).await?;
    
    Ok(Json(coord))
}

async fn weather_handler(
    Path(city): Path<String>,
    State(pool): State<Pool<Postgres>>
) -> Result<WeatherDisplay> {  //Result<Json<Main>> {
    println!("->> {:<12} Weather Handler", "HANDLER");

    let weather = fetch_weather(&city, &pool).await?;

    let display = WeatherDisplay {
        main: weather,
        name: city,
    };
    
    Ok(display)
}
