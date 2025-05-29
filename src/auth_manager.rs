use kiteconnect::connect::KiteConnect;
use reqwest::Client;
use sha2::{Digest, Sha256};
use std::{env, time::{Duration, Instant}};

pub struct AuthManager {
    pub kite: KiteConnect,
    pub api_key: String,
    pub api_secret: String,
    pub access_token: Option<String>,
    pub token_expiry: Option<Instant>
}

impl AuthManager {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            kite: KiteConnect::new(&api_key, ""),
            api_key,
            api_secret,
            access_token: Some(env::var("ACCESS_TOKEN").expect("access token not set!")),
            token_expiry: None
        }
    }

    pub fn set_access_token(&mut self, access_token: String) {
        self.kite = KiteConnect::new(&self.api_key, &access_token);
        self.access_token = Some(access_token);
        self.token_expiry = Some(Instant::now() + Duration::from_secs(12 * 60 * 60));
    }

    pub fn is_token_valid(&mut self) -> bool {
        match self.token_expiry {
            Some(expiry) => Instant::now() < expiry,
            None => false
        }
    }

    pub fn get_kite(&mut self) -> &KiteConnect {
        &self.kite
    }

    pub fn get_login_url(&mut self) -> String {
        self.kite.login_url()
    }

    /*pub async fn get_access_token(&mut self, api_key: &str, api_secret: &str, request_token: &str) -> Result<String, Box<dyn std::error::Error>> {
        let checksum_input = format!("{}{}{}", api_key, request_token, api_secret);
        let checksum = format!("{:x}", Sha256::digest(checksum_input.as_bytes()));
        let client = Client::new();
        let response = client.post("https://api.kite.trade/session/token")
        .form(&[
            ("api_key", api_key),
            ("api_secret", api_secret),
            ("checksum", &checksum)
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

        if let Some(data) = response.get("data") {
            if let Some(access_token) = data.get("access_token") {
                let token_str = access_token.as_str().unwrap().to_string();
                self.access_token = Some(token_str.clone());
                println!("access token successfully stored: {}", token_str);
                return Ok(token_str);
            }
            else {
                Err("access token wasn't stored".into())
            }
        }
        else {
            Err("Failed to get access token from API response".into())
        }
        
        /*let access_token = response["data"] ["access_token"].as_str().unwrap();
        Ok(access_token.to_string())*/
    }*/

    pub async  fn generate_session(&mut self, request_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("== GENERATING SESSION ==");
        println!("Request token: {}", request_token);
        println!("API key: {}", self.api_key);
        println!("API secret: {}", self.api_secret);
        println!("== END DEBUG ==");

        let checksum_input = format!("{}{}{}", self.api_key, request_token, self.api_secret);
        let checksum = format!("{:x}", Sha256::digest(checksum_input.as_bytes()));
        println!("Checksum: {:?}", checksum);

        let client = Client::new();
        let response = client.post("https://api.kite.trade/session/token")
        .form(&[
            ("api_key", &self.api_key),
            ("api_secret", &self.api_secret),
            ("checksum", &checksum)
        ])
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

        if let Some(data) = response.get("data") {
            if let Some(access_token) = data.get("access_token") {
                let token_str = access_token.as_str().unwrap().to_string();
                self.access_token = Some(token_str.clone());
                println!("access token successfully stored: {}", token_str);
                return Ok(());
            }
            else {
                Err("access token wasn't stored".into())
            }
        }
        else {
            Err("Failed to get access token from API response".into())
        }
    }
}