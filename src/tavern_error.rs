pub enum TavernErrorType {
    GeneralError,
    ControllerError,
    RepositoryError,
    UnknownError
}

#[derive(Debug, Clone)]
/// An all consuming error type for the common library. That should interface with
/// anything wrong that happens when using the TavernCommon
pub struct TavernError {
    /// The message to be passed through TavernCommon and its APIs
    pub message: String,
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

    /// Creates a new `TavernError` with a default message
    pub fn default() -> Self {
        Self { message: "This is a default error. The developer does not know what happened. Please make an issue on github.".to_string(),
            error_type: TavernErrorType::GeneralError,
        }
    }

    /// Returns the error message as a string
    pub fn err(&self) -> String { self.message }

    /// Appends the given message to the existing error message and returns the modified `TavernError`
    pub fn and(&self, m: String) -> Self { 
        self.message += format!(" !AND! {}", m).as_str();
        *self 
    }
}

impl std::fmt::Display for TavernError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TavernError Occured {}", self.message)
    }
}
