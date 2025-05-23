use std::collections::HashMap;

use crate::{data_structures::{AppState, ErrorResponse, TradeInstruction, TradeResponse}, trade_executor::TradeExecutor};
use actix_web::{web::{self}, HttpResponse};
use chrono::Utc;
use kiteconnect::connect::KiteConnect;
use serde_json::json;

pub async fn execute_trade(app_state: web::Data<AppState>, instruction: web::Json<TradeInstruction>) -> HttpResponse {

    let mut auth_manager = app_state.auth_manager.lock().unwrap();

    if !auth_manager.is_token_valid() {
        return HttpResponse::Unauthorized().json(ErrorResponse {
            status: "Error".to_string(),
            message: "Authentication token invalid or not found..".to_string()
        });
    }

    let kite = auth_manager.get_kite();
    let mut final_instruction = instruction.0.clone();

    if final_instruction.symbol == "BEST PERFORMER" {
        let mut market_data = app_state.market_data.lock().unwrap();
        match market_data.best_performer(final_instruction.timeframe.unwrap_or(20)).await {
            Ok(symbol) => {
                final_instruction.symbol = symbol;
                return HttpResponse::Ok().json(final_instruction.symbol.clone())
            },
            Err(e) => {
                return HttpResponse::BadRequest().json(ErrorResponse {
                status: "Error".to_string(),
                message: format!("Failed to find out best performant stock: {}", e)
                });
            }
        }
    }

    let mut exeucutor = TradeExecutor::new(kite);

    match exeucutor.execute_instructions(&final_instruction) {
        Ok(order_id) => {
            HttpResponse::Ok().json(TradeResponse {
                order_id,
                status: "Success".to_string(),
                message: format!("Order placed successfully for: {}", final_instruction.symbol),
                symbol: final_instruction.symbol,
                quantity: instruction.quantity,
                price: instruction.limit_price.expect("Cannot find the limit price.."),
                timestamp: Utc::now().to_rfc3339()
            })
        },
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
            status: "Error".to_string(),
            message: format!("Failed to execute order: {}", e)
            });
        }
    }
}

pub async fn get_login_url(app_state: web::Data<AppState>) -> HttpResponse {
    let mut auth_manager = app_state.auth_manager.lock().unwrap();
    let login_url = auth_manager.get_login_url();

    HttpResponse::Ok().json(json!({
        "login_url": login_url
    }))
}

pub async fn auth_callback(app_state: web::Data<AppState>, query: web::Query<HashMap<String, String>>) -> HttpResponse {
    if let Some(token) = query.get("access token") {
        let mut auth_manager = app_state.auth_manager.lock().unwrap();
        match auth_manager.generate_session(token) {
            Ok(_) => {
                if let Some(access_token) = &auth_manager.access_token {
                    let mut market_data = app_state.market_data.lock().unwrap();
                    let kite_clone = KiteConnect::new(&auth_manager.api_key, &access_token);
                    market_data.set_kite(kite_clone);
                    market_data.initialize_ticker(&auth_manager.api_key, &access_token);
                }
                else {
                    return HttpResponse::BadRequest().json(json!({
                        "status": "Unsuccessful".to_string(),
                        "message": "Invaild access token received!".to_string()
                    }));
                }
                
                HttpResponse::Ok().json(json!({
                    "status": "Successful".to_string(),
                    "message": "Authentication successfull".to_string()
                }))
            },
            Err(e) => HttpResponse::BadRequest().json(json!({
                "status": "Unsuccessful".to_string(),
                "message": format!("Authentical failed: {}", e)
            }))
        }
    }
    else {
        return HttpResponse::BadRequest().json(json!({
            "status": "Unsuccessful".to_string(),
            "message": "Access token not found!".to_string()
        }));
    }
}