use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

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
pub fn argon_encrypt_salt(to_be_encrypted: String) -> Result<String, argon2::Error> {
    let to_be_encrypted_as_bytes = to_be_encrypted.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2_config = Argon2::default();

    let hash = argon_encrypt_salt.hash_password(to_be_encrypted_as_bytes, &salt)?.to_string();
    Ok(hash)
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
pub fn validate_password(password: String, hash: &str) -> Result<bool, argon2::Error> {
    let password_as_bytes = password.as_bytes();
    let parsed_hash = PasswordHash::new(hash)?;

    let argon2_config = Argon2::default();
    Ok(argon2_config.verify_password(password_as_bytes, &parsed_hash).is_ok())
}

