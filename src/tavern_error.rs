use std::fmt;

use actix_web::http::StatusCode;

#[derive(Debug)]
pub enum TavernError {
    ProcessingError(String),
    ServerError(StatusCode, String),
}

impl Default for TavernError {
    fn default() -> Self {
        Self::ProcessingError("An error occured while processing the data".to_string())
    }
}

impl fmt::Display for TavernError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TavernError::ProcessingError(e) => 
                writeln!(f, "ProcessingError: {}", e),
            TavernError::ServerError(sc, e) =>
                writeln!(f, "ServerError {}: {}", sc, e)
        }
    }
}

impl std::error::Error for TavernError {}

impl TavernError {

    pub fn err(&self) -> String {
        if let TavernError::ProcessingError(e) = self {
            format!("Processing Error occured: {}", e)      
        } else if let TavernError::ServerError(sc, e) = self {
            format!("Server Error {}: {}", sc, e)
        } else {
            "An error occured, yet the reason is unknown or the error was not initialized.".to_string()
        }
    }
}

pub type TRes<T> = Result<T, TavernError>;

pub use TavernError::{ServerError, ProcessingError};
