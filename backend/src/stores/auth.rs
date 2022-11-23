use std::collections::HashMap;
use std::ops::Add;

use std::time::{Duration, SystemTime};

use tokio::sync::RwLock;

use crate::utils::{create_character_set, create_random_string};

/// Default store credentials
const DEFAULT_USERNAME: &str = "admin";
const DEFAULT_PASSWORD: &str = "admin";

const ENV_USERNAME_KEY: &str = "DRIVEPI_USERNAME";
const ENV_PASSWORD_KEY: &str = "DRIVEPI_PASSWORD";

/// The time it takes for an access token to expire in
/// seconds. (In this case 5 Hours)
const TOKEN_EXPIRY_TIME: u64 = 60 * 60 * 5;

/// The character length to generate the tokens with
const TOKEN_LENGTH: usize = 48;

/// Struct for storing the API authentication credentials
/// and a map of tokens to their expiry times.
pub struct AuthStore {
    username: String,
    password: String,

    tokens: RwLock<HashMap<String, SystemTime>>,
}

/// Structure for a token to expiry time mapping
pub struct TokenData {
    pub token: String,
    pub expiry_time: SystemTime,
}

impl AuthStore {
    /// Creates a new instance of the auth store using the
    /// provided username and password as the credentials.
    pub fn new() -> AuthStore {
        let username = std::env::var(ENV_USERNAME_KEY).unwrap_or(String::from(DEFAULT_USERNAME));
        let password = std::env::var(ENV_PASSWORD_KEY).unwrap_or(String::from(DEFAULT_PASSWORD));

        Self {
            username,
            password,
            tokens: RwLock::new(HashMap::new()),
        }
    }

    /// Checks whether the provided username and password
    /// match the credentials stored in the store
    pub fn is_credentials(&self, username: &String, password: &String) -> bool {
        self.username == *username && self.password == *password
    }

    /// Retrieves the expiry time for the provided token
    pub async fn get_token_expiry(&self, token: &str) -> Option<SystemTime> {
        let tokens = &*self.tokens.read().await;
        match tokens.get(token) {
            None => None,
            Some(expiry_time) => Some(expiry_time.clone()),
        }
    }

    /// Removes the provided token from the valid tokens map
    pub async fn remove_token(&self, token: &str) {
        let tokens = &mut *self.tokens.write().await;
        tokens.remove(token);
    }

    /// Adds the provided token into the tokens map with its
    /// provided expiry time.
    async fn add_token(&self, token: String, expiry_time: SystemTime) {
        let tokens = &mut *self.tokens.write().await;
        tokens.insert(token, expiry_time);
    }

    /// Checks whether the token exists in the tokens map and
    /// will remove the token if the token is expired returning
    /// whether the token is valid
    pub async fn check_token(&self, token: &str) -> bool {
        let expiry_time = self.get_token_expiry(token).await;
        match expiry_time {
            Some(expiry_time) => {
                let current_time = SystemTime::now();
                if current_time >= expiry_time {
                    // Remove expired token
                    self.remove_token(token).await;
                    false
                } else {
                    true
                }
            }
            None => false,
        }
    }

    /// Creates a new unique token and inserts it into the tokens map
    pub async fn create_token(&self) -> TokenData {
        let character_set = create_character_set();
        loop {
            let token = create_random_string(&character_set, TOKEN_LENGTH);
            let expiry_time = self.get_token_expiry(&token).await;

            // If the token exists continue attempting to create tokens
            if expiry_time.is_some() {
                continue;
            }

            let current_time = SystemTime::now();

            let expiry_duration = Duration::from_secs(TOKEN_EXPIRY_TIME);
            let expiry_time = current_time.add(expiry_duration);

            self.add_token(token.clone(), expiry_time.clone()).await;

            return TokenData { token, expiry_time };
        }
    }
}
