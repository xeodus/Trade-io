use std::{env, io, sync::{Arc, Mutex}};

use actix_web::{web, App, HttpServer};
use api_manager::{auth_callback, execute_trade, get_login_url};
use auth_manager::AuthManager;
use data_structures::AppState;
use market_data::MarketData;

pub mod auth_manager;
pub mod data_structures;
pub mod market_data;
pub mod trade_executor;
pub mod api_manager;

#[actix_web::main]

async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY").expect("API key not set!");
    let api_secret = env::var("API_SECRET").expect("API secret not found!");

    let auth_manager = AuthManager::new(api_key, api_secret);
    let market_data = Arc::new(Mutex::new(MarketData::new()));

    let app_state = web::Data::new(AppState {
        auth_manager: Mutex::new(auth_manager),
        market_data: market_data.clone()
    });

    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/trade", web::post().to(execute_trade))
            .route("/auth", web::get().to(get_login_url))
            .route("/auth/callback", web::get().to(auth_callback))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}