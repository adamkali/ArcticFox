//! This is Taver's authentication. It is a way to make sure that users are logged in when they are
//! accessing the application and important data

use crate::data_structures::model::Model;
use super::encryption;
use chrono::{
    DateTime,
    offset::TimeZone,
    Utc
};
use uuid::{uuid, Uuid};
use crate::tavern_error::TavernError;

/// internal Type for development ease
type Res<T> = std::result::Result<T, TavernError>;

/// a unary struct that communicates to the ecosystem that this user has unrestricted access to the
/// entire ecosystem.
pub struct Admin;

/// Depending on the level that the user has (0, 1, 2, 3); the user is able to access
/// specified api Access points, or different features until the expiration date is reached.
pub struct Access {
    pub level:              u8,
    pub expiration_date:    DateTime<Utc>,
}

/// Depending on the level that the user has (1, 2, 3); the user is able to access specified api
/// access points, or different features for the lifetime of the web app.
pub struct LTAccess {
    pub level:              u8,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
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
    Access(u8, DateTime<Utc>),
    LTAccess(u8),
    Admin,
}

impl AccessRole {
    pub fn get_role(
        level: Option<u8>, 
        expiration: Option<DateTime<Utc>>
    ) -> Res<AccessRole> { 
        let mut temp: AccessRole = AccessRole::None;
        
        match level {
            Some(l) => {
                if l == 4 as u8 {
                    temp = AccessRole::Admin;
                } else if l == 3 as u8 {
                    match expiration {
                        Some(dt) => { temp = AccessRole::Access(l, dt); },
                        None => { temp = AccessRole::LTAccess(l); }
                    }
                }
            },
            _ => {}
        }
        Ok(temp)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
/// A model that holds authentication and contact information.
/// it will be using two things:
///     - 1. making the id hold a guid for salting the hash.
///     - 2. having the password be held by the username and password
///          used during signup.
///     - 3. have an enum for the access allowed within Tavern.
///          see [`AccessRole`]
pub struct Token {
    pub userid:         String,
    pub userhash:       String,
    pub useraccess:     AccessRole, 
    pub username:       String,
    pub useremail:      String
}

impl Token {
    /// Initializes the token with specified values. Will throw up an error to be dealt with on the api side
    /// rather than the in this common crate.
    ///
    /// ## Arguments
    ///
    /// * `username`:   `String` a string that the user chooses and should be unique as well. The
    ///                 The uniqueness will already be tested befor this even gets created, so that
    ///                 does not need to be checked in the crate.
    ///
    /// * `useremail`:  `String` a string that should be unique but just like username, that will
    ///                 checked here. This will also be used to actually send emails to the users.
    ///
    /// * `password`:   `String` a string that will have check for appropriate characters for
    ///                 purposes.
    ///
    /// ## Returns
    pub fn init(
        username: String, useremail: String, password: String
    ) -> Res<Self> {
        let userid = Uuid::new_v4();
        let salt = format!("{}-{}", username, userid.to_string()).to_string();
        let token = match encryption::argon_encrypt_salt(password) {
            Ok(h) => Self {
                userid: userid.to_string(),
                userhash: h,
                username,
                useremail,
                useraccess: AccessRole::None
            },
            Err(e) => e
        };

        Ok(token)
    }
}
