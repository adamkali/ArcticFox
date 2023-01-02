use std::fmt;


#[derive(Clone, Copy, Default)]
pub enum TavernErrorType {
    /// An error occurred in the general code
    GeneralError,
    /// An error occurred in the controller code
    ControllerError,
    /// An error occurred in the repository code
    RepositoryError,
    /// An unknown error occurred
    #[default]
    UnknownError
}


/// An all consuming error type for the common library. That should interface with
/// anything wrong that happens when using the TavernCommon
pub struct TavernError {
    /// A message describing the error
    pub message: String,
    /// The type of error that occurred
    pub error_type: TavernErrorType,
}


impl TavernError {
    /// Creates a new `TavernError` with the given message
    pub fn new(m: String) -> Self {
        Self { 
            message: m,
            error_type: TavernErrorType::GeneralError,
        } 
    }

    /// Returns the error message as a string
    pub fn err(&self) -> String { self.message.to_string() }

    /// Appends the given message to the existing error message and returns the modified `TavernError`
    pub fn and(&self, m: String) -> Self { 
        Self {
            message: self.message.to_owned() + &m,
            error_type: self.error_type,
        }
    }
}

impl std::default::Default for TavernError {
    fn default() -> Self {
       Self {
           message: "An unforseen error occurred.".to_string(),
           error_type: TavernErrorType::default(),
       } 
    }
}

impl std::fmt::Display for TavernError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "TavernError Occured {}", self.message) 
    } 
}

impl fmt::Debug for TavernError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TavernError")
            .field("message", &self.message)
            .field("",
                    if let TavernErrorType::GeneralError = &self.error_type {
                        b"General Error"
                    }  else if let TavernErrorType::ControllerError = &self.error_type {
                        b"Controller Error"
                    } else if let TavernErrorType::RepositoryError = &self.error_type {
                        b"Repository Error"
                    } else {
                        b"Unknown Error"
                    }
            )
            .finish()
    }
}

impl Clone for TavernError {
    fn clone(&self) -> Self {
        Self { 
            message: self.message.to_owned(), 
            error_type: self.error_type 
        }
    }
} 

/// A type alias for a `Result` type with `TavernError` as the `Err` variant
pub type TRes<T> = Result<T, TavernError>;
