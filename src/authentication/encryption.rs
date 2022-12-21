use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use crate::tavern_error::TavernError;

/// A private type for internal use. Just makes life easier.
type Result<T> = std::result::Result<T, TavernError>;

/// argon_encrypt_salt is a function that will take in a string that needs encrypted, a salting
/// string, and then use the rust implementation of argon2 to salt the password and return the encrypted string
///
/// ## Arguments
///
/// * to_be_encrypted: `String`
/// * salt: `&str`
///
/// ## Returns
///
/// `hash`: and encrypted string that is encrypted using Argon2.
///
/// ## Notes
///
/// `hash` can be verified later, by using the [`validate_password`] function. 
///
pub fn argon_encrypt_salt(to_be_encrypted: String) -> Result<String> {
    let to_be_encrypted_as_bytes = to_be_encrypted.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2_config = Argon2::default();

    match argon2_config.hash_password(to_be_encrypted_as_bytes, &salt) {
        Ok(h) => { h.to_string()},
        _ => { TavernError::new("There was an encryption error.".to_string()) }
    };
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
/// returns a Result<bool, argon2::Error>
///
/// ## Errors
///
/// will return an error that needs to be handeled in the API.
pub fn validate_password(password: String, hash: &str) -> Result<bool> {
    let password_as_bytes = password.as_bytes();
    let parsed_hash = PasswordHash::new(hash);

    let argon2_config = Argon2::default();
    match argon2_config.verify_password(password_as_bytes, &parsed_hash) {
        Ok(m) => m,
        Err(e) => TavernError::new(format!("Could not verify password because of: {}", e.to_string()))
    }
}

/// is_valid_password: a function to use when signing up to see if the password from a
/// SignupRequest is strong enough. This will check for:
///     - 1. A capital letter.
///     - 2. A lowercase letter.
///     - 3. A number
///     - 4. A symbol that does not alter sql queries.
///     - 5. A minimum of 8 characters.
///     - 6. A maximum of 32 characters.
///     - 7. No 3 repeating characters.
///
/// ## Arguments 
///
/// * `password`: a password string passed in by the SignupRequest to check if the password is
/// strong enough
///
/// ## Returns 
///
/// returns a bool depending if the conditions are met.
pub fn is_valid_password(password: &str) -> (bool, Option<String>) {
    if password.len() < 8 || password.len() > 32 {
        return (false, Some("Password must have between 8 and 32 characters to be valid.".to_string()));
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_number    = false;
    let mut has_symbol    = false;

    let mut last_seen: char;
    let mut last_seen_number = 0;
    for c in password.chars() {
        if last_seen == c && last_seen_number == 2 { 
             return ( false, Some("A password should not have 3 consecutive same characters.".to_string()));
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
            c == '&' || c == '*' || c == '(' || 
            c == ')' || c == '_' || c == '-' || 
            c == '+' || c == '=' {
            has_symbol = true
        }
    }

    if !has_uppercase || !has_lowercase || !has_number || !has_symbol {
        return (false, Some("The password must have an Uppercase letter, a Lowercase letter, a Symbol (!@#$%^&*()_-+=), or a Number in order to be a valid password.".to_string()));
    }

    (true, None)
}
