use crate::tavern_error::TavernError;
use super::model::Model;

use std::{
    convert::Infallible, 
    pin::Pin, 
    task::Context,
};
use core::task::Poll;
use actix_web::{
    web::Bytes,
    body::{MessageBody, BodySize},
};

use serde::Serialize;

static SUCCESSFUL_MESSAGE: &str = "The Request is Successful";

#[derive(Serialize)]
pub struct RespondentStruct<T: Model> {
    pub data: T,
    pub message: String,
}

pub enum Respondent<T: Model> {
    Successful(T),
    Unsuccessful(T, TavernError),
}

impl<T: Model> std::marker::Unpin for Respondent<T> {}

impl<T: Model> Respondent<T> {
    pub fn run<F: FnOnce(T) -> Result<T, TavernError>>(&mut self, f: F) {
        match self {
            Respondent::Successful(data) => {
                match f(data.clone()) {
                    Ok(new_data) => {
                        *self = Respondent::Successful(new_data);
                    },
                    Err(err) => {
                        let tavern_error: TavernError = TavernError::new(err.err());
                        *self = Respondent::Unsuccessful(data.clone(), tavern_error);
                    }
                }
            },
            Respondent::Unsuccessful(_, _) => (),
        }
    }
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
    /// `Respondent` to a string for async requests.
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

