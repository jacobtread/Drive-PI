use actix_web::web::Json;
use rand::{Rng, thread_rng};

pub type JsonResult<R, E> = Result<Json<R>, E>;

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
            Some(char) => { result.push(*char) }
            None => {}
        }
    }

    return result;
}