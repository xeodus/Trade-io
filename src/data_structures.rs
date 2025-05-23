use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

use crate::{auth_manager::AuthManager, market_data::MarketData};

#[derive(Debug, Deserialize, Clone)]
pub struct TradeInstruction {
    pub action: String,
    pub symbol: String,
    pub exchange: String,
    pub quantity: u32,
    pub price_type: String,
    pub limit_price: Option<f64>,
    pub stop_loss: Option<f64>,
    pub target: Option<f64>,
    pub order_id: Option<String>,
    pub timeframe: Option<u64>
}

#[derive(Debug, Serialize)]
pub struct TradeResponse {
    pub order_id: String,
    pub status: String,
    pub message: String,
    pub symbol: String,
    pub quantity: u32,
    pub price: f64,
    pub timestamp: String
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String
}

pub struct AppState {
    pub auth_manager: Mutex<AuthManager>,
    pub market_data: Arc<Mutex<MarketData>>
}