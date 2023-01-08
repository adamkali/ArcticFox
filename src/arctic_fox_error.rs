use std::fmt;

use actix_web::http::StatusCode;

#[derive(Debug, Clone)]
pub enum ArcticFoxError {
    Forbidden(String),
    Unauthorized(String),
    Uncommon(String, StatusCode),
    UserError(String, StatusCode),
    ServerError(String, StatusCode),
}

impl Default for ArcticFoxError {
    fn default() -> Self {
        todo!();
    }
}

impl fmt::Display for ArcticFoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ArcticFoxError::Forbidden(e) => 
                writeln!(f, "Forbidden: {}", e),
            ArcticFoxError::Unauthorized(e) =>
                writeln!(f, "Unauthorized: {}", e),
            ArcticFoxError::UserError(e, sc) =>
                writeln!(f, "{} Error: {}", sc, e),
            ArcticFoxError::Uncommon(e, sc) =>
                writeln!(f, "{} Error: {}", sc, e),
            ArcticFoxError::ServerError(e, sc) =>
                writeln!(f, "{} Error: {}", sc, e),
        }
    }
}

impl std::error::Error for ArcticFoxError {}

impl ArcticFoxError {

    pub fn err(&self) -> String {
        match self {
            ArcticFoxError::Forbidden(e) => 
                format!("Forbidden: {}", e),
            ArcticFoxError::Unauthorized(e) =>
                format!("Unauthorized: {}", e),
            ArcticFoxError::UserError(e, sc) =>
                format!("{} Error: {}", sc, e),
            ArcticFoxError::Uncommon(e, sc) =>
                format!("{} Error: {}", sc, e),
            ArcticFoxError::ServerError(e, sc) =>
                format!("{} Error: {}", sc, e),
        }
    }

    pub fn to_status_code(&self) -> StatusCode {
        match self {
            ArcticFoxError::Forbidden(_e) => 
               StatusCode::FORBIDDEN,
            ArcticFoxError::Unauthorized(_e) =>
               StatusCode::UNAUTHORIZED,
            ArcticFoxError::UserError(_e, sc) =>
                *sc,
            ArcticFoxError::Uncommon(_e, sc) =>
                *sc,
            ArcticFoxError::ServerError(_e, sc) =>
                *sc,
        }
    }
}

pub type TRes<T> = Result<T, ArcticFoxError>;

pub use ArcticFoxError::{
    Forbidden,
    Unauthorized,
    Uncommon,
    UserError,
    ServerError
};
