//! This is Taver's authentication. It is a way to make sure that users are logged in when they are
//! accessing the application and important data

use crate::data_structures::model::Model;
use super::encryption;
use chrono::{
    DateTime,
    Utc
};
use uuid::Uuid;
use crate::tavern_error::TavernError;

type Res<T> = Result<T, TavernError>;

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
    /// Returns the appropriate `AccessRole` for the given user level and expiration date.
    ///
    /// ## Arguments
    ///
    /// * `level`:       `Option<u8>` representing the level of access for the user. 
    ///                 The possible values are `1`, `2`, `3`, or `4`. If `None`, 
    ///                 `AccessRole::None` is returned.
    ///
    /// * `expiration`:  `Option<DateTime<Utc>>` representing the expiration date for the user's access. 
    ///                 If `Some`, `AccessRole::Access` is returned with the given level and expiration date. 
    ///                 If `None`, `AccessRole::LTAccess` is returned with the given level.
    ///
    /// ## Returns
    ///
    /// `Res<AccessRole>`: An `Ok` variant with the appropriate `AccessRole` if the provided level is valid. 
    /// An `Err` variant with an error message otherwise.
    pub fn get_role(
        level: Option<u8>, 
        expiration: Option<DateTime<Utc>>
    ) -> Res<AccessRole> { 
        let temp: AccessRole;
        
        match level {
            Some(l) => {
                if l == 4_u8 {
                    temp = AccessRole::Admin;
                } else if (1_u8..=3_u8).contains(&l) {
                    match expiration {
                        Some(dt) => { temp = AccessRole::Access(l, dt); },
                        None => { temp = AccessRole::LTAccess(l); }
                    }
                } else {
                    temp = AccessRole::LTAccess(0);
                }
            },
            _ => { temp = AccessRole::None; },
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
    pub id:             String,
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
    /// # Arguments
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
    /// # Returns
    ///
    /// If successful, returns a `Self` object. If there is an error, returns a `TavernError` object.
    pub fn init(
        &self, 
        username: String, 
        useremail: String,
        password: String
    ) -> Res<Self> {
        let id = Uuid::parse_str(&self.id);

        match id {
           Ok(i) => {
               if i.is_nil() {
                   let userid = Uuid::new_v4().to_string();
                   match encryption::argon_encrypt_salt(password) {
                       Ok(h) => Ok(Self {
                           id:         i.to_string(),
                           userid,
                           userhash:   h,
                           username,
                           useremail,
                           useraccess: AccessRole::None
                       }),
                       Err(e) => Err(TavernError::new(e.err())),
                   }
               } else {
                    let id = Uuid::new_v4().to_string();
                    let userid = Uuid::new_v4().to_string();
                    match encryption::argon_encrypt_salt(password) {
                        Ok(h) => Ok(Self {
                            id,
                            userid,
                            userhash:   h,
                            username,
                            useremail,
                            useraccess: AccessRole::None
                        }),
                        Err(e) => Err(TavernError::new(e.err())),
                    }
               }
           },
           Err(_) => Err(TavernError { 
               message: "An Error occurred in processing Token initialization".to_string(), 
               error_type: crate::tavern_error::TavernErrorType::GeneralError, 
           })
        }
    }
}

impl std::default::Default for Token {
    /// Creates a new `Token` instance with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tavernWebCommon::models::user::Token;
    ///
    /// let token = Token::default();
    /// ```
    fn default() -> Self {
        Self {
            id:         Uuid::new_v4().to_string(),
            userid:     Uuid::new_v4().to_string(),
            userhash:   "".to_string(),
            username:   "".to_string(),
            useremail:  "".to_string(),
            useraccess: AccessRole::None
        }
    }
}

impl Model for Token {
    /// Creates a new `Token` instance.
    ///
    /// # Arguments
    ///
    /// * `id`: An optional `String` representing the id of the `Token`.
    ///
    /// # Examples
    ///
    /// ```
    /// use TavernWebCommon::models::user::{Model, Token};
    ///
    /// let token_with_id = Token::new(Some("12345".to_string()));
    /// let token_without_id = Token::new(None);
    /// ```
    fn new(id: Option<String>) -> Self {
        match id {
            Some(i) => {
                Self {
                    id: i,
                    ..Self::default()
                }
            },
            None => {
                Self::default()
            },
        }
    }

    /// Calculates the size of the `Token` instance in bytes.
    ///
    /// ## Returns
    ///
    /// Returns a `u64` representing the size of the `Token` instance in bytes.
    fn size(&self) -> u64 {
        let mut access_role_string = String::new();
        if let AccessRole::None = self.useraccess {
            access_role_string = "Free Tier".to_string();
        } else if let AccessRole::Admin = self.useraccess {
            access_role_string = "Admin".to_string();
        } else if let AccessRole::Access(i, d) = self.useraccess {
            if i == 1_u8 {
                access_role_string = format!("Adventurer Tier {}", d);
            } else if i == 2_u8 {
                access_role_string = format!("Hero Tier {}", d);
            } else if i == 3_u8 {
                access_role_string = format!("Legend Tier {}", d);
            }
        } else if let AccessRole::LTAccess(i) = self.useraccess {
            if i == 1_u8 {
                access_role_string = "Eternal Adventurer Tier".to_string();
            } else if i == 2_u8 {
                access_role_string = "Eternal Hero Tier".to_string();
            } else if i == 3_u8 {
                access_role_string = "Legend Tier".to_string();
            }
        }

        (
            self.id.len() 
                + self.userid.len() 
                + self.userhash.len() 
                + self.username.len() 
                + self.useremail.len()
                + access_role_string.len()
        ) as u64
    }
}
