#![allow(unused)]

pub mod error;
pub mod web;
pub mod api;

use std::net::SocketAddr;
use axum::{Router, extract::State, routing::{get, post}, response::{IntoResponse, Html, Redirect}, Form, http::Response};
use error::Result;
use sqlx::PgPool;
use askama::{ Template };
use serde::Deserialize;


#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[derive(Deserialize)]
struct CityForm {
    city: String
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("Couldn't find the db url");
    let db_pool = PgPool::connect(&database_url)
        .await?;

    let main_router = Router::new()
        .route("/", get(|| async { Index {} }))
        .route("/foo", post(redirect_handler))
        .merge(web::weather_routes::weather_routes(db_pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> LISTENING on {}", addr);
    axum::Server::bind(&addr)
        .serve(main_router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn redirect_handler(Form(page): Form<CityForm>) -> impl IntoResponse {
    println!("->> {:<12} Redirect Handler", "HANDLER");

    let url = format!("/weather/{}", page.city);
    Redirect::to(&url)
}
