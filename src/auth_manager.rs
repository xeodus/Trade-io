use kiteconnect::connect::KiteConnect;
use std::time::{Duration, Instant};

pub struct AuthManager {
    pub kite: KiteConnect,
    pub api_key: String,
    pub api_secret: String,
    pub access_token: Option<String>,
    pub token_expiry: Option<Instant>
}

impl AuthManager {
    pub fn new(&self) -> Self {
        Self {
            kite: KiteConnect::new(&self.api_key, ""),
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
            access_token: None,
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

    pub fn generate_session(&mut self, request_token: &str) -> Result<(), Box<dyn std::error::Error>> {
        let users = self.kite.generate_session(request_token, &self.api_secret)?;
        let access_token = match users.get("access token") {
            Some(token) => token.as_str().unwrap_or("").to_string(),
            None => return Err("No access token was found".into())
        };

        if access_token.is_empty() {
            return Err("Empty access token received".into());
        }

        self.set_access_token(access_token);
        Ok(())
    }
}