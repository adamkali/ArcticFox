use argon2::{self, Config, ThreadMode};

use crate::tavern_error::{TavernError, TRes};

use uuid::Uuid;

/// Using Argon2 v1.0.0
///
/// the fn used here is hash_encoded
/// 
/// ## Arguments
///
/// * to_be_encrypted: [String] a string like a password that needs to be encrypted.
///
/// ## Returns 
/// 
/// Returns a Result of String or a TavernError.
/// 
///
pub fn argon_encrypt_salt(to_be_encrypted: String) -> TRes<String> {
    let config = Config {
        thread_mode: ThreadMode::Parallel,
        lanes: 4,
        ..Config::default()
    };

    // Encrypt in 
    match argon2::hash_encoded(to_be_encrypted.as_bytes(),
                                    Uuid::new_v4().as_bytes(),
                                    &config)
    {
        Ok(hash) => Ok(hash),
        Err(_) => { Err( 
            TavernError { 
                message: "There was a problem encrypting the password".to_string(), 
                error_type: crate::tavern_error::TavernErrorType::GeneralError 
            }
        )},
    }
}

/// validate_password: a function that takes in a password and a hash. From there we can validate
/// the password matches the hash,
///
/// ## Arguments
///
/// * `password`: a password from a login request.
/// * `hash`: hash stored for the specific username passed into the LoginRequest.
///
/// ## Returns
///
/// returns a Result<bool, TavernError>
///
/// ## Errors
///
/// will return an error that needs to be handeled in the API. Will return a GeneralError
pub fn validate_password(password: String, hash: &str) -> TRes<bool> {
    match argon2::verify_encoded(hash, password.as_bytes()) {
        Ok(b) => { Ok(b)},
        Err(_) => { Err(
            TavernError { 
                message: "There was aproblem trying to verify the password".to_string(), 
                error_type: crate::tavern_error::TavernErrorType::GeneralError 
            }
        )}
    }
}
/// Validates whether a given password meets certain requirements.
///
/// # Arguments
///
/// * `password` - A string slice representing the password to validate.
///
/// # Returns
///
/// Returns a `Result` containing a boolean value indicating whether the password is valid or not.
/// If the password is invalid, the `Err` variant will contain a `TavernError` with a message
/// describing the specific validation errors./
///
/// # Examples
///
/// ```
/// use tavernError;
/// let password = "P@55word";
/// let is_valid = is_valid_password(password);
/// assert!(is_valid.is_ok());
/// ```
///
/// ```
/// use TavernError;
/// let password = "p@55word";
/// let is_valid = is_valid_password(password);
/// assert!(is_valid.is_err());
/// ```
///
/// ```
/// use TavernError;
/// let password = "P@55word ";
/// let is_valid = is_valid_password(password);
/// assert!(is_valid.is_err());
/// ```
///
/// ```
/// use TavernError;
/// let password = "P@55wor d";
/// let is_valid = is_valid_password(password);
/// assert!(is_valid.is_err());
/// ```
///
/// ```
/// use TavernError;
/// let password = "P@55worddd";
/// let is_valid = is_valid_password(password);
/// assert!(is_valid.is_err());
/// ```
pub fn is_valid_password(password: &str) -> TRes<bool> {
    let mut error_message: Option<String> = None;

    if password.len() < 8 || password.len() > 32 {
        error_message = Some("- The error must be between 8 and 32 characters".to_string()); 
    }

    let mut has_uppercase    = false;
    let mut has_lowercase    = false;
    let mut has_number       = false;
    let mut has_symbol       = false;
    let mut last_seen: char  = ' ';
    let mut last_seen_number = 0;

    for c in password.chars() {
        if c == ' ' {
            let place_holder = "- There should not be a space in a password.".to_string();
            error_message = 
                match error_message {
                    Some(e) => {
                        let err = format!(
                            "{}\n{}\n",
                            e, place_holder 
                        );
                        Some(err)
                    },
                    None => Some(place_holder)
                };
            break
        }

        if last_seen == c && last_seen_number == 2 { 
            let place_holder = "- There should not be 3 consective characters that are the same.".to_string();
            error_message = 
                match error_message {
                    Some(e) => {
                        let err = format!(
                            "{}\n{}\n",
                            e, place_holder 
                        );
                        Some(err)
                    },
                    None => Some(place_holder)
                };
            break
        } else if last_seen == c {
            last_seen_number += 1;
        } else if last_seen != c {
            last_seen = c;
            last_seen_number = 0;
        }

        if c.is_uppercase() {
            has_uppercase = true;
        } else if c.is_lowercase() {
            has_lowercase = true;
        } else if c.is_numeric() {
            has_number = true;
        } else if c == '!' || c == '@' || c == '#' || 
            c == '$' || c == '%' || c == '^' || 
            c == '&' || c == '*' || 
            c == '_' || c == '-' || 
            c == '+' || c == '=' {
            has_symbol = true
        } else {
            let place_holder 
                = format!("- There is an invalid character in the password: {}.", c);
            error_message = 
                match error_message {
                    Some(e) => {
                        let err = format!(
                            "{}\n{}\n",
                            e, place_holder 
                        );
                        Some(err)
                    },
                    None => Some(place_holder)
                };
            break
        }
    }

    if !has_uppercase || !has_lowercase || !has_number || !has_symbol {
        let place_holder = "The password must contain at least a capital letter, a lowercase letter and a numer.".to_string();

        error_message = 
        match error_message {
            Some(e) => {
                let err = format!(
                    "{}{}\n",
                    e, place_holder 
                    );
                Some(err)
            },
            None => Some(place_holder)
        };
    }

    match error_message {
        Some(message) => Err(TavernError {
            message,
            error_type: crate::tavern_error::TavernErrorType::GeneralError, 
        }),
        None => Ok(true)
    }
    
}
