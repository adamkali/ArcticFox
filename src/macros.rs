

#[macro_export]
macro_rules! unauthorized {
    ($($arg:tt)*) => {
        ArcticFoxError::Unauthorized(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! forbidden {
    ($($arg:tt)*) => {
        ArcticFoxError::Forbidden(format!($($arg)*))
    }
}

#[macro_export]
macro_rules! server_error {
    ($status_code:expr, $($arg:tt)*) => {
        ArcticFoxError::ServerError(format!($($arg)*), $status_code)
    }
}

#[macro_export]
macro_rules! user_error {
    ($status_code:expr, $($arg:tt)*) => {
        ArcticFoxError::UserError(format!($($arg)*), $status_code)
    }
}

#[macro_export]
macro_rules! uncommon_error {
    ($status_code:expr, $($arg:tt)*) => {
        ArcticFoxError::UncommonError(format!($($arg)*), $status_code)
    }
}

#[macro_export]
macro_rules! bond {
    ($cub:expr) => { ArcticFox::Successful($cub) }
}


#[macro_export]
macro_rules! adopt {
    ($adopted:expr) => { ArcticFox::Successful(AdoptedCub::default($adopted)) }
}
