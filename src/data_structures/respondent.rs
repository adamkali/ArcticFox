use crate::tavern_error::{
    TavernError,
    ServerError,
    ProcessingError,
};
use super::model::Model;

use std::{
    convert::Infallible, 
    pin::Pin, 
    task::Context,
};
use core::task::Poll;
use actix_web::{
    web::Bytes,
    HttpResponse,
    body::{MessageBody, BodySize},
};

use serde::Serialize;

use crate::processing;

static SUCCESSFUL_MESSAGE: &str = "The Request is Successful";

/// The struct that gets serialized at the end of the request's processing.
#[derive(Serialize)]
pub struct RespondentStruct<T: Model> {
    /// The data that will be sent back to the user that uses the `Model` trait.
    pub data: T,
    /// A message sent back to the client side. If Successful then the `SUCCESSFUL_MESSAGE` will be
    /// sent back to the client. If Unsuccessful then the api will send back the message absorbed
    /// by TavernError with the data. 
    pub message: String,
}

/// Respondent is a Monadic operater that uses Successful or Unsuccessful.
pub enum Respondent<T: Model> {
    /// `Successful` tells the api that the operation is `Ok(...)`
    Successful(T),
    /// `Unsuccessful` tells the api that the operation produced an error. In this case TavernError
    /// will absorb the error and will be processed when converting `Respondent` into
    /// `RespondentStruct`.
    Unsuccessful(T, TavernError),
}

impl<T: Model + Serialize + Clone> std::clone::Clone for Respondent<T> {
    fn clone(&self) -> Self { todo!() }
}

impl<T: Model> std::marker::Unpin for Respondent<T> {}

impl<T: Model + Serialize > Respondent<T> {
    /// Main crutch of the entire Respondent Monad. if the function that is inside of the `run(|t|
    /// => ...)` then the Respondent will still be Successful. If not then the Respondent will be
    /// Unsuccessful. If an operation is acted upon the Respondent while Unsuccessful nothin will
    /// happen.
    ///
    /// # Arguments
    ///
    /// * f: FnOnce(T) a function that will operate on the stored by the Respondent.
    ///
    /// # Example
    /// 
    /// ```
    /// fn update_user(user: &mut User) -> Result<(), TavernError> {
    ///     // do some work to update the user
    ///     Ok(())
    /// }
    /// 
    /// let mut response = DetailedResponse::Successful(User::default());
    /// response.run(|user| {
    ///     // update the user and handle any errors that may occur
    ///     if let Err(err) = update_user(user) {
    ///         *response = DetailedResponse::Unsuccessful(user.clone(), err);
    ///     }
    /// });
    /// ```
    pub fn run<F: FnOnce(T) -> Result<T, TavernError>>(&mut self, f: F) {
        match self {
            Respondent::Successful(data) => {
                match f(data.clone()) {
                    Ok(new_data) => {
                        *self = Respondent::Successful(new_data);
                    },
                    Err(err) => {
                        let tavern_error: TavernError 
                            = processing!("An error occured: {}", err);
                        *self = Respondent::Unsuccessful(data.clone(), tavern_error);
                    }
                }
            },
            Respondent::Unsuccessful(_, _) => (),
        }
    }

    /// The `Respondent`'s is_success functor. It is used to tell Tavern's ecosystem that all
    /// operations done onto the `Respondent` up until the call of is_success has not resulted in
    /// an error.
    pub fn is_success(&self) -> bool { matches!(self, Respondent::Successful(_t)) }

    /// The `Respondent`'s not_success functor. It is used to tell Tavern's ecosystem that at some
    /// point an operation done inside the `Respondent`'s perview was unsuccessful, and resulted in
    /// an error.
    pub fn not_success(&self) -> bool { matches!(self, Respondent::Unsuccessful(_t, _e)) }

    //pub fn respond(&self) -> HttpResponse { 
    //    
    //    match self {
    //        Successful(t) => HttpResponse::Ok().body(Successful(t.clone())),
    //        Unsuccessful(_, err) => {
    //            let container: Self = (*self).clone();
    //            match err {
    //                ServerError(s, _m) => 
    //                    HttpResponse::build(*s).body(container),
    //                ProcessingError(_m) => HttpResponse::build(
    //                    actix_web::http::StatusCode::INTERNAL_SERVER_ERROR)
    //                    .body(container)
    //            }
    //        }
    //    }
    //}  
}


impl<T: Model + Serialize + Clone> MessageBody for Respondent<T> {
    type Error = Infallible;

    fn size(&self) -> BodySize {

        let mut payload_bytes: u64 = 0_u64;

        match &self {
            Self::Successful(data) => {
                payload_bytes += data.size() + SUCCESSFUL_MESSAGE.len() as u64;
            },
            Self::Unsuccessful(data, tavern_error) => {
                payload_bytes += data.size() + tavern_error.err().len() as u64;
            }
        }

        BodySize::Sized(payload_bytes)
    }

    /// Polls the next chunk of bytes to send in the response body, serializing the
    /// `Respondent` to a string for async requests. This uses the `RespondentStruct`
    /// to serialize the `Respondent`
    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let data = self.get_mut();

        let payload_string = match data {
            Respondent::Successful(d) => {
                serde_json::to_string_pretty(
                    & RespondentStruct {
                        data: d.clone(),
                        message: SUCCESSFUL_MESSAGE.to_string(),
                    }
                )
            },
            Respondent::Unsuccessful(d, e) => {
                serde_json::to_string_pretty(
                    & RespondentStruct {
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

pub use Respondent::{Successful, Unsuccessful};
