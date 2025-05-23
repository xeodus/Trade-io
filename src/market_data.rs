use std::{cmp::Ordering, collections::{HashMap, HashSet}, sync::{Arc, Mutex}, time::Duration};
use anyhow::Ok;
use chrono::Utc;
use kiteconnect::{connect::KiteConnect, ticker::{KiteTicker, WebSocketHandler, KiteTickerHandler}};

pub struct MarketData {
    kite: Option<KiteConnect>,
    ticker: Option<KiteTicker>,
    live_prices: HashMap<String, f64>,
    watchlist: HashSet<String>
}

#[derive(Debug)]
pub struct MarketDataHandler {
    prices: Arc<Mutex<HashMap<String, f64>>>
}

impl KiteTickerHandler for MarketDataHandler {
    fn on_open<T>(&mut self, ws: &mut WebSocketHandler<T>)
    where T: KiteTickerHandler
    {
        let ws_ = ws.subscribe(vec![123456]);
        println!("WebSocket connection established: {:?}", ws_);
    }

    fn on_ticks<T>(&mut self, _ws: &mut WebSocketHandler<T>, ticks: Vec<serde_json::Value>)
    where T: KiteTickerHandler
    {
        let mut prices = self.prices.lock().unwrap();
        println!("ticks: {:?}", ticks);

        for tick in ticks {
            if let (Some(symbol), Some(last_price)) = (
                tick.get("symbol").and_then(|s| s.as_str()),
                tick.get("last_price").and_then(|p| p.as_f64())
            ) {
                prices.insert(symbol.to_string(), last_price);
            }
            println!("Fellow on_ticks callback for prices: {:?}", prices);
        }
    }

    fn on_close<T>(&mut self, _ws: &mut WebSocketHandler<T>)
    where T: KiteTickerHandler
    {
        println!("WebSocket connection closed.");
    }

    fn on_error<T>(&mut self, _ws: &mut WebSocketHandler<T>)
    where T: KiteTickerHandler
    {
        println!("WebSocket connection error occured..");
    }
}

impl MarketData {
    pub fn new() -> Self {
        Self {
            kite: None,
            ticker: None,
            live_prices: HashMap::new(),
            watchlist: ["RELIANCE", "TCS", "HDFCBANK", "INFY", "SBIN", "TATAMOTORS", "ITC"]
            .iter().map(|x| x.to_string()).collect()
        }
    }

    pub fn initialize_ticker(&mut self, api_key: &str, access_token: &str) {
        if let Some(kite) = &self.kite {
            let instruments = kite.instruments(Some("TATAM0TORS"));
            let mut tokens = Vec::new();
            for instrument in instruments.into_iter() {
                if let (Some(symbol), Some(last_token)) = (
                    instrument.get("symbol").and_then(|s| s.as_str()),
                    instrument.get("last_token").and_then(|t| t.as_f64())
                ) {
                    if self.watchlist.contains(symbol) {
                        tokens.push(last_token);
                    }
                }
            }

            if !tokens.is_empty() {
                let shared_prices = Arc::new(Mutex::new(self.live_prices.clone()));
                let handler = MarketDataHandler { prices: shared_prices.clone() };
                let mut ticker = KiteTicker::new(api_key, access_token);
                let _ = ticker.connect(handler, None);
                self.ticker = Some(ticker);
            }
        }
    }

    pub fn set_kite(&mut self, kite: KiteConnect) {
        self.kite = Some(kite);
    }

    pub fn get_quote(&mut self, symbol: &str)  -> Result<f64, anyhow::Error> {
        if let Some(price) = self.live_prices.get(symbol) {
            return Ok(*price);
        }

        if let Some(kite) = &self.kite {
            let quotes = kite.quote((&[symbol]).to_vec())?;
            if let Some(quote) = quotes.get(symbol) {
                if let Some(last_price) =  quote.get("symbol")
                .and_then(|f| f.as_f64()) {
                    return Ok(last_price);
                }
                else {
                    return Err(anyhow::anyhow!("Unable to fetch the last price for: {}", symbol));
                }
            }
            else {
                return Err(anyhow::anyhow!("Unable to fetch the quote for: {}", symbol));
            }
        }
        else {
            return Err(anyhow::anyhow!("Unable to get quote for: {}", symbol));
        }
    }

    pub fn get_instrumental_token(&self, symbol: &str) -> Result<String, anyhow::Error> {
        if let Some(kite) = &self.kite {
            let instruments = kite.instruments(Some("ITC"));
            let token = String::from("");
            for ele in instruments.into_iter() {
                if let (Some(sym), Some(token)) = (
                    ele.get("symbol").and_then(|s| s.as_str()),
                    ele.get("token").and_then(|t| t.as_str())
                ) {
                    if sym == symbol {
                        return Ok(token.to_string());
                    }
                }
                else {
                    return Err(anyhow::anyhow!("Unable to get the token for: {}", symbol));
                }
            }
            Ok(token)
        }
        else {
            return Err(anyhow::anyhow!("Unable to connect kite for: {}", symbol));
        }
    }

    pub fn historical_data(& self, symbol: &str, from: i64, to: i64) -> Result<Vec<serde_json::Value>, anyhow::Error> {
        if let Some(kite) = &self.kite {
            let token = self.get_instrumental_token(symbol)?;
            let data = kite.historical_data(&token, &from.to_string(), &to.to_string(), "minute", "true".into())?;
            Ok(vec![data])
        }
        else {
            return Err(anyhow::anyhow!("Unable to get historical data from KiteConnect for: {}", symbol));
        }
    }

    pub async fn best_performer(&mut self, timeframe_units: u64) -> Result<String, anyhow::Error> {
        let now = Utc::now();
        let from = now - Duration::from_secs(timeframe_units);
        let mut performances = Vec::new();

        for symbol in &self.watchlist {
            let data = self.historical_data(&symbol, from.timestamp(), now.timestamp())?;
            if data.len() >= 2 {
                let first_ = &data[0];
                let last_ = &data[data.len() - 1];
                let first_price = first_.get("close").and_then(|f| f.as_f64()).unwrap_or(0.0);
                let last_price = last_.get("close").and_then(|f| f.as_f64()).unwrap_or(0.0);

                if first_price > 0.0 {
                    let performance = (last_price - first_price) / first_price * 100.0;
                    performances.push((symbol.clone(), performance));
                }
                else {
                    return Err(anyhow::anyhow!("Invalid price for symbol: {}", symbol));
                }
            }
            else {
                return Err(anyhow::anyhow!("Invalid data received for: {}", symbol));
            }
        }
        // Sort in order by descending performance
        performances.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

        if let Some((symbol, performance)) = performances.first() {
            println!("Best performer: {} ({}%)", symbol, performance);
            Ok(symbol.clone())
        }
        else {
            return Err(anyhow::anyhow!("No performance data is available.."));
        }
    }
}