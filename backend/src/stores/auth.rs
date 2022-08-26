use std::collections::HashMap;
use std::ops::Add;
use std::sync::{LockResult, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime};
use log::error;
use rand::distributions::{Alphanumeric, DistString};
use rand::{Rng, thread_rng};

pub struct AuthStore {
    username: String,
    password: String,
    pub tokens: RwLock<HashMap<String, SystemTime>>,
}

const TOKEN_EXPIRY_TIME_MILLIS: u64 = 1000 * 60 * 60 * 5;

impl AuthStore {
    pub fn new(username: String, password: String) -> AuthStore {
        AuthStore {
            username,
            password,
            tokens: RwLock::new(HashMap::new()),
        }
    }

    pub fn is_valid_credentials(&self, username: &String, password: &String) -> bool {
        self.username == *username && self.password == *password
    }

    fn remove_token(&mut self, token: &String) {
        let tokens_lock = self.tokens.write();
        match tokens_lock {
            Ok(mut tokens) => {
                tokens.remove(token);
            }
            Err(_) => error!("Failed to acquire lock to remove tokens")
        }
    }

    fn insert_token(&mut self, token: String, expiry_time: SystemTime) {
        let tokens_lock = self.tokens.write();
        match tokens_lock {
            Ok(mut tokens) => {
                tokens.insert(token, expiry_time);
            }
            Err(_) => error!("Failed to acquire lock to remove tokens")
        }
    }

    pub fn check_token(&mut self, token: &String) -> bool {
        let expiry_time = self.get_token_expiry(token);
        match expiry_time {
            Some(expiry_time) => {
                let current_time = SystemTime::now();
                if current_time >= expiry_time {
                    self.remove_token(token);
                    false
                } else {
                    true
                }
            }
            None => false
        }
    }

    pub fn get_token_expiry(&self, token: &String) -> Option<SystemTime> {
        let tokens = self.tokens.read();
        match tokens {
            Ok(tokens) => match tokens.get(token) {
                Some(expiry_time) => Some(*expiry_time),
                None => None
            },
            Err(_) => {
                error!("Failed to acquire lock of tokens");
                None
            }
        }
    }

    pub fn create_token(&mut self) -> Option<(String, SystemTime)> {
        let length = 24;
        let alphabet = "abcdefghijklmnopqrstuvwxyz";
        let num = "0123456789";

        let char_count = alphabet.len() * 2 + num.len();

        let mut chars: Vec<char> = Vec::with_capacity(char_count);
        alphabet.chars().for_each(|char| {
            chars.push(char);
            chars.push(char.to_ascii_uppercase())
        });
        num.chars().for_each(|char| chars.push(char));
        let mut rng = thread_rng();

        loop {
            let mut result = String::with_capacity(length);
            for _ in 0..length {
                let char_index = rng.gen_range(0..char_count);
                let char = chars.get(char_index)
                    .expect("Char didn't exist");
                result.push(*char)
            }

            let expiry_time = self.get_token_expiry(&result);
            if expiry_time.is_some() {
                continue;
            }

            let expiry_duration = Duration::from_millis(TOKEN_EXPIRY_TIME_MILLIS);
            let expiry_time = SystemTime::now().add(expiry_duration);

            self.insert_token(
                result.clone(),
                expiry_time.clone(),
            );

            return Some((result, expiry_time));
        }
    }
}