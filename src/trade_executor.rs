use std::collections::HashMap;
use std::result::Result::Ok;
use kiteconnect::connect::KiteConnect;
use crate::data_structures::TradeInstruction;

pub struct TradeExecutor<'a> {
    pub kite: &'a KiteConnect
}

impl<'a> TradeExecutor<'a> {
    pub fn new(kite: &'a KiteConnect) -> Self {
        Self { kite }
    }

    pub fn execute_instructions(&mut self, instruction: &TradeInstruction) -> Result<String, anyhow::Error> {
        match instruction.action.as_str() {
            "buy" => self.place_buy_order(instruction),
            "sell" => self.place_sell_order(instruction),
            "cancel" => self.cancel_order(instruction),
            _ => return Err(anyhow::anyhow!("Unsupported action: {}", instruction.action))
        }
    }

    pub fn place_buy_order(&mut self, instruction: &TradeInstruction) -> Result<String, anyhow::Error> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", instruction.symbol.clone());
        params.insert("exchange", instruction.exchange.clone());
        params.insert("transaction type", "BUY".to_string());
        params.insert("quantity", instruction.quantity.to_string());
        params.insert("product", "CNC".to_string());

        let order_type = match instruction.price_type.as_str() {
            "MARKET" => {
                params.insert("order type", "MARKET".to_string())
            },
            "lIMIT" => {
                if let Some(limit_price) = instruction.limit_price  {
                    params.insert("order type", limit_price.to_string());
                }
                else {
                    return Err(anyhow::anyhow!("Limit price required for limit order"));
                }
                params.insert("order type", "LIMIT".to_string())
            },
            _ => return Err(anyhow::anyhow!("Unsupported price type received: {}", instruction.price_type))
        };

        let price_ = instruction.limit_price.map(|p| p.to_string());
        let price_ref = price_.as_deref();
        let stop_loss = instruction.stop_loss.map(|s| s.to_string());

        let response = self.kite.place_order(
            &instruction.exchange,
            &instruction.symbol,
            "BUY",
            &instruction.quantity.to_string(),
            "regular",
            price_.as_deref(),
            Some("CNC"),
            order_type.as_deref(),
            Some("DAY"),
            None,
            None,
            None,
            stop_loss.as_deref(),
            None,
            price_ref
        )?;

        let order_id = match response.get("order_id") {
            Some(id) => id.to_string(),
            None => return Err(anyhow::anyhow!("Cannot get a valid order id"))
        };
        Ok(order_id)
    }

    fn place_sell_order(&mut self, instruction: &TradeInstruction) -> Result<String, anyhow::Error> {
        let mut params = HashMap::new();
        params.insert("tradingsymbol", instruction.symbol.clone());
        params.insert("exchange", instruction.exchange.clone());
        params.insert("quantity", instruction.quantity.to_string());
        params.insert("transaction type", "SELL".to_string());
        params.insert("product", "CNC".to_string());

        let order_type = match instruction.price_type.as_str() {
            "MARKET" => params.insert("order type", "MARKET".to_string()),
            "LIMIT" => {
                if let Some(limit_price) = instruction.limit_price {
                    params.insert("order type", limit_price.to_string())
                }
                else {
                    return Err(anyhow::anyhow!("Valid price not received.."));
                }
            },
            _ => return Err(anyhow::anyhow!("Cannot fetch order type.."))
        };

        let price_ = instruction.limit_price.map(|p| p.to_string());
        let price_ref = price_.as_deref();
        let stop_loss = instruction.stop_loss.map(|s| s.to_string());

        let response = self.kite.place_order(
            &instruction.exchange,
            &instruction.symbol,
            "SELL",
            &instruction.quantity.to_string(),
            "regular",
            price_.as_deref(),
            Some("CNC"),
            order_type.as_deref(),
            Some("DAY"),
            None,
            None,
            None,
            stop_loss.as_deref(),
            None,
            price_ref
        )?;

        let order_id = match response.get("order_id") {
            Some(id) => id.to_string(),
            None => {
                return Err(anyhow::anyhow!("Cannot get a valid order id.."));
            }
        };
        Ok(order_id)
    }

    fn cancel_order(&mut self, instruction: &TradeInstruction) -> Result<String, anyhow::Error> {
        if let Some(order_id) = &instruction.order_id {
            self.kite.cancel_order(order_id, "regular", Some(order_id))?;
            Ok(order_id.to_string())
        }
        else {
            return Err(anyhow::anyhow!("Cannot cancel order.."));
        }
    }
}