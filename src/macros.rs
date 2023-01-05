
#[macro_export]
macro_rules! processing {
    ($($arg:tt)*) => {
        TavernError::ProcessingError(format!($($arg)*))
    } 
}

#[macro_export]
macro_rules! serving {
    ($status_code:expr, $($arg:tt)*) => {
        TavernError::ServerError($status_code, format!($($arg)*))
    }
}
