use std::collections::HashMap;
use std::ops::Add;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, SystemTime};
use actix_web::web::Data;
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

// Errors
pub enum AuthError {
    ReadFailure,
    AddFailure,
    RemoveFailure,
}

// Type alias for results that can result in AuthError's
type AuthResult<T> = Result<T, AuthError>;

/// Structure for a token to expiry time mapping
pub struct TokenData {
    pub token: String,
    pub expiry_time: SystemTime,
}

pub type AuthStoreData = Data<Arc<Mutex<AuthStore>>>;

impl AuthStore {
    /// Creates a new instance of the auth store using the
    /// provided username and password as the credentials.
    pub fn create() -> Self {
        let username = std::env::var(ENV_USERNAME_KEY)
            .unwrap_or(String::from(DEFAULT_USERNAME));

        let password = std::env::var(ENV_PASSWORD_KEY)
            .unwrap_or(String::from(DEFAULT_PASSWORD));

        Self {
            username,
            password,
            tokens: RwLock::new(HashMap::new()),
        }
    }

    pub fn to_safe(self) -> Arc<Mutex<Self>> {
        return Arc::new(Mutex::new(self));
    }

    /// Checks whether the provided username and password
    /// match the credentials stored in the store
    pub fn is_credentials(
        &self,
        username: &String,
        password: &String,
    ) -> bool {
        self.username == *username
            && self.password == *password
    }

    /// Retrieves the expiry time for the provided token
    fn get_token_expiry(
        &self,
        token: &String,
    ) -> AuthResult<Option<SystemTime>> {
        match self.tokens.read() {
            Ok(tokens) => {
                match tokens.get(token) {
                    None => Ok(None),
                    Some(expiry_time) => Ok(Some(expiry_time.clone()))
                }
            }
            Err(_) => Err(AuthError::ReadFailure)
        }
    }

    /// Removes the provided token from the valid tokens map
    fn remove_token(
        &mut self,
        token: &String,
    ) -> AuthResult<()> {
        match self.tokens.write() {
            Ok(mut tokens) => {
                tokens.remove(token);
                Ok(())
            }
            Err(_) => Err(AuthError::RemoveFailure)
        }
    }

    /// Adds the provided token into the tokens map with its
    /// provided expiry time.
    fn add_token(
        &mut self,
        token: String,
        expiry_time: SystemTime,
    ) -> AuthResult<()> {
        match self.tokens.write() {
            Ok(mut tokens) => {
                tokens.insert(token, expiry_time);
                Ok(())
            }
            Err(_) => Err(AuthError::AddFailure)
        }
    }

    /// Checks whether the token exists in the tokens map and
    /// will remove the token if the token is expired returning
    /// whether the token is valid
    pub fn check_token(
        &mut self,
        token: &String,
    ) -> AuthResult<bool> {
        let expiry_time = self.get_token_expiry(token)?;
        match expiry_time {
            Some(expiry_time) => {
                let current_time = SystemTime::now();
                Ok(if current_time >= expiry_time {
                    // Remove expired token
                    self.remove_token(token)?;
                    false
                } else {
                    true
                })
            }
            None => Ok(false)
        }
    }

    /// Creates a new unique token and inserts it into the tokens map
    pub fn create_token(&mut self) -> AuthResult<TokenData> {
        let character_set = create_character_set();
        loop {
            let token = create_random_string(&character_set, TOKEN_LENGTH);
            let expiry_time = self.get_token_expiry(&token)?;

            // If the token exists continue attempting to create tokens
            if expiry_time.is_some() {
                continue;
            }

            let current_time = SystemTime::now();

            let expiry_duration = Duration::from_secs(TOKEN_EXPIRY_TIME);
            let expiry_time = current_time.add(expiry_duration);

            self.add_token(
                token.clone(),
                expiry_time.clone(),
            )?;

            return Ok(TokenData {
                token,
                expiry_time,
            });
        }
    }
}