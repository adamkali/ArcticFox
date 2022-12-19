//! This is Taver's authentication. It is a way to make sure that users are logged in when they are
//! accessing the application and important data

use crate::data_structures::Model;
use chrono::{
    DateTime,
    offset::TimeZone
};

/// a unary struct that communicates to the ecosystem that this user has unrestricted access to the
/// entire ecosystem.
pub struct Admin;

/// Depending on the level that the user has (0, 1, 2, 3); the user is able to access
/// specified api Access points, or different features until the expiration date is reached.
pub struct Access {
    pub level:              u8,
    pub expiration_date:    DateTime<UTC>,
}

/// Depending on the level that the user has (1, 2, 3); the user is able to access specified api
/// access points, or different features for the lifetime of the web app.
pub struct LTAccess {
    pub level:              u8,
}

/// An enum that defines what the user is able to do. If a user making a request has a None as the
/// AccessRole than the request is denied for most requests. There are 3 levels of access for what
/// a user can aceess.
///     - 1. None: The user has made an account, but needs to verify the account.
///     - 2. Level (0): The user has made an account and is free.
///     - 3. Level (1): The user is a premium account that has unlimited characters, and plots that
///          they can make.
///     - 4. Level (2): The user is a premium account that has everything from Access(1) and that
///          they can have access to any number of groups to play and chat with.
///     - 5. Level (3): The user is access to all features
///     - 6. `Access(u8, DateTime<UTC>)`: Shows that the premium access will end at some point
///     - 7. `LTAccess(u8)`: Shows that the user has premium access for life.
///     - 8. Admin: Has access to all api endpoints.
pub enum AccessRole {
    None,
    Access(u8, DateTime<TimeZone>),
    LTAccess(u8),
    Admin,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq)]
/// A model that holds authentication and contact information.
/// it will be using two things:
///     - 1. making the id hold a guid for salting the hash.
///     - 2. having the password be held by the username and password
///          used during signup.
///     - 3. have an enum for the access allowed within Tavern.
///          see [`AccessRole`]
pub struct Token {
    pub id:         String,
    pub hash:       String,
    pub access:     AccessRole, 
}
