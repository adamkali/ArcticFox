use crate::arctic_fox_error::ArcticFoxError;

use super::cub::Cub;

use std::{
    convert::Infallible, 
    pin::Pin, 
    task::Context,
    future::Future,
};

use core::task::Poll;
use actix_web::{
    web::Bytes,
    body::{MessageBody, BodySize},
};

use serde::Serialize;

use crate::{unauthorized, bond};


static SUCCESSFUL_MESSAGE: &str = "The Request is Successful";

/// The struct that gets serialized at the end of the request's processing.
#[derive(Serialize)]
pub struct ArcticFoxStruct<T: Cub> {
    /// The data that will be sent back to the user that uses the `Cub` trait.
    pub data: T,
    /// A message sent back to the client side. If Successful then the `SUCCESSFUL_MESSAGE` will be
    /// sent back to the client. If Unsuccessful then the api will send back the message absorbed
    /// by ArcticFoxError with the data. 
    pub message: String,
}

/// ArcticFox is a Monadic operater that uses Successful or Unsuccessful.
pub enum ArcticFox<T: Cub> {
    /// `Successful` tells the api that the operation is `Ok(...)`
    Successful(T),
    /// `Unsuccessful` tells the api that the operation produced an error. In this case ArcticFoxError
    /// will absorb the error and will be processed when converting `ArcticFox` into
    /// `ArcticFoxStruct`.
    Unsuccessful(T, ArcticFoxError),
}

impl<T: Cub> std::marker::Unpin for ArcticFox<T> {}
impl<T: Cub> std::clone::Clone for ArcticFox<T> {
    fn clone(&self) -> Self {
        match self { Successful(t) => bond!(t.clone()), Unsuccessful(t, e) => Unsuccessful(t.clone(), e.clone()) }
    }
}

impl<T: Cub + Serialize > ArcticFox<T> {
    /// Main crutch of the entire ArcticFox Monad. if the function that is inside of the `run(|t|
    /// => ...)` then the ArcticFox will still be Successful. If not then the ArcticFox will be
    /// Unsuccessful. If an operation is acted upon the ArcticFox while Unsuccessful nothin will
    /// happen.
    ///
    /// # Arguments
    ///
    /// * f: FnOnce(T) a function that will operate on the stored by the ArcticFox.
    ///
    /// # Example
    /// 
    /// ```
    /// fn update_user(user: &mut User) -> Result<(), ArcticFoxError> {
    ///     // do some work to update the user
    ///     Ok(())
    /// }
    /// 
    /// let mut response = ArcticFox::Successful(User::default());
    /// response.run(|user| {
    ///     // update the user and handle any errors that may occur
    ///     if let Err(err) = update_user(user) {
    ///         *response = ArcticFox::Unsuccessful(user.clone(), err);
    ///     }
    /// });
    /// ```
    pub fn run<F>(&mut self, f: F) -> &mut Self 
        where F: FnOnce(T) -> Result<T, ArcticFoxError>
    {
        match self {
            ArcticFox::Successful(data) => {
                match f(data.clone()) {
                    Ok(new_data) => {
                        *self = ArcticFox::Successful(new_data);
                    },
                    Err(err) => {
                        let fox_error: ArcticFoxError 
                            = unauthorized!("An error occured: {}", err);
                        *self = ArcticFox::Unsuccessful(data.clone(), fox_error);
                    }
                }
            },
            ArcticFox::Unsuccessful(_, _) => (),
        }
        self
    }

    /// asunc version of `ArcticFox::run()`. See that implementation for more infromation.
    pub async fn async_run<F, Fut>(&mut self, f: F) -> &mut Self
        where F: FnOnce(T) -> Fut, Fut: Future<Output = Result<T, ArcticFoxError>>
    {
        match self {
            Successful(data) => {
                let new_data = f(data.clone()).await;
                match new_data {
                    Ok(new_data) => { *self = Successful(new_data); self },
                    Err(err) => {
                        let fox_error: ArcticFoxError
                            = unauthorized!("An error occurred: {}", err);
                        *self = Unsuccessful(data.clone(), fox_error);
                        self
                    }
                }
            },
            Unsuccessful(_,_) => self,
        }
    }

    /// The `ArcticFox`'s is_success functor. It is used to tell the caller that everything up to
    /// the point of calling `is_success()` has not absorbed any unsuccessful operations.
    pub fn is_success(&self) -> bool { matches!(self, ArcticFox::Successful(_t)) }

    /// The `ArcticFox`'s is_success functor. It is used to tell the caller that everything up to
    /// the point of calling `not_success()` has absorbed an unsuccessful operation and the value
    /// stored in the ArcticFox has not mutated since then.
    pub fn not_success(&self) -> bool { matches!(self, ArcticFox::Unsuccessful(_t, _e)) }

    pub fn successful(&self) -> Self {
        match self {
            Successful(_t) => { self.clone() },
            _ => panic!("`success()` was called on a unsuccessful ArcticFox. As such the program paniced.")
        } 
    }

    /// Simple method to provide the value encased in the monad.
    pub fn freeze(&self) -> (T, Option<ArcticFoxError>){
        match self {
            Successful(t) => { (t.clone(), None) }
            Unsuccessful(t, e) => { return (t.clone(), Some(e.clone())) }
        }
    }
}


impl<T: Cub + Serialize + Clone> MessageBody for ArcticFox<T> {
    type Error = Infallible;

    fn size(&self) -> BodySize {

        let mut payload_bytes: u64 = 0_u64;

        match &self {
            Self::Successful(data) => {
                payload_bytes += data.size() + SUCCESSFUL_MESSAGE.len() as u64;
            },
            Self::Unsuccessful(data, fox_error) => {
                payload_bytes += data.size() + fox_error.err().len() as u64;
            }
        }

        BodySize::Sized(payload_bytes)
    }

    /// Polls the next chunk of bytes to send in the response body, serializing the
    /// `ArcticFox` to a string for async requests. This uses the `ArcticFoxStruct`
    /// to serialize the `ArcticFox`
    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let data = self.get_mut();

        let payload_string = match data {
            ArcticFox::Successful(d) => {
                serde_json::to_string_pretty(
                    & ArcticFoxStruct {
                        data: d.clone(),
                        message: SUCCESSFUL_MESSAGE.to_string(),
                    }
                )
            },
            ArcticFox::Unsuccessful(d, e) => {
                serde_json::to_string_pretty(
                    & ArcticFoxStruct {
                        data: d.clone(),
                        message: e.err()
                    }
                )
            }
        };

        let payload_bytes = Bytes::from(payload_string
                                            .unwrap_or_else(|_|
                                                "Something went wrong...".to_string()));

        Poll::Ready(Some(Ok(payload_bytes)))
    }
}

pub use ArcticFox::{Successful, Unsuccessful};
