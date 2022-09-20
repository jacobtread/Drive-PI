pub mod drives;
pub mod files;

use actix_web::web::Json;
use log::warn;
use rand::{thread_rng, Rng};

const ENV_PORT_KEY: &str = "DRIVEPI_PORT";
const DEFAULT_PORT: u16 = 80;

pub type JsonResult<R, E> = Result<Json<R>, E>;
pub type JsonEmpty<E> = Result<Json<()>, E>;

/// Creates the charset used for generating random strings. This
/// function is separate so that create_random_string can be used
/// many times in a loop without reallocating this vec
pub fn create_character_set() -> Vec<char> {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";
    return charset.chars().collect();
}

/// Creates a random string of the provided length from randomly
/// select chars in the provided charset
pub fn create_random_string(charset: &Vec<char>, length: usize) -> String {
    let mut rng = thread_rng();
    let mut result = String::with_capacity(length);

    let char_count = charset.len();

    for _ in 0..length {
        let char_index = rng.gen_range(0..char_count);
        match charset.get(char_index) {
            Some(char) => result.push(*char),
            None => {}
        }
    }

    return result;
}

pub fn get_env_port() -> u16 {
    let port_env = std::env::var(ENV_PORT_KEY);
    if let Ok(port_raw) = port_env {
        if let Ok(port) = port_raw.parse::<u16>() {
            port
        } else {
            warn!(
                "Port provided as {} is not a valid port defaulting to {}",
                port_raw, DEFAULT_PORT
            );
            DEFAULT_PORT
        }
    } else {
        DEFAULT_PORT
    }
}

pub fn ok_json<V, E>(value: V) -> Result<Json<V>, E> {
    return Ok(Json(value));
}

pub fn ok_json_empty<E>() -> Result<Json<()>, E> {
    return Ok(Json(()));
}
