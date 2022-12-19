/// DetailedResponse

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
    http::StatusCode
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
/// A struct that represents a detailed response to a request.
///
/// # Example
///
/// ## Example for an Ok Response
/// ```rust
/// use tavern_common::{DetailedResponse, Model};
/// use actix_web::HttpResponse;
///
/// struct DumbStruct {
///     name: String,
/// }
///
/// impl Model for DumbStruct {
///     pub fn new(&self, id: Option<String>) -> Self {
///         match (id) {
///             Some(n) => Self { name: id.unwrap() }
///             None => Self { name: "default" }
///         } 
///     }
///
///     pub fn size(&self) -> u64 {
///         self.name.len() as u64
///     }
/// }
///
/// #[get("/foo/bar/{name}")]
/// async fn foo_bar_name_get(name: web::Path<String>) -> HttpResponse {
///     let mut response: DetailedResponse<DumbStruct>
///         = DetailedResponse<DumbStruct>::new();
///     let obj = DumbStruct::new(name);
///     
///     HttpResponse::Ok().body(response.ok(obj, "This is a sample message"));
/// }
/// 
/// ```
///
/// ## Example for an Error Response
/// ```rust
/// use tavern_common::{DetailedResponse, Model}
/// use actix_web::HttpResponse;
///
/// struct DumbStruct {
///     name: String,
/// }
///
/// impl Model for DumbStruct {
///     pub fn new(&self, id: Option<String>) -> Self {
///         match (id) {
///             Some(n) => Self { name: id.unwrap() }
///             None => Self { name: "default" }
///         } 
///     }
///
///     pub fn size(&self) -> u64 {
///         self.name.len() as u64
///     }
/// }
///
/// #[get("/foo/bar/{name}")]
/// async fn foo_bar_name_get(name: web::Path<String>) -> HttpResponse {
///     let mut response: DetailedResponse<DumbStruct>
///         = DetailedResponse<DumbStruct>::new();
///     let obj = DumbStruct::new(name);
///     
///     HttpResponse::Ok().body(
///         response.consume_error(
///             StatusCode::UNAUTHORIZED,
///             obj,
///             "You do not have the right, oh you do not have the right!"
///         )
///     );
/// }
/// 
/// ```
///
pub struct DetailedResponse<T: Model + Serialize + Clone> {
    /// The data payload of the response to be consumed by the Client
    pub data: T,
    /// A part of the response that tells the client whether the request was successful and operate
    /// the data from there.
    pub successful: bool,
    /// A message to be sent to the client. It could be an error so that a message can be displayed
    /// in the UI, or a message explaining that something was saved.
    pub message: String,
}

impl<T: Model + Serialize + Clone> DetailedResponse< T> {
    /// Creates a default `DetailedResponse` with a default type passed into it that uses 
    pub fn new() -> Self {
        Self {
            data: T::new(None),
            successful: false,
            message: "".to_string(),
        }
    }

    /// Creates a new `DetailedResponse` with a success status and the given data and message
    pub fn ok(&mut self, data: T, message: Option<String>) -> Self {
        let mut response: Self = DetailedResponse::new();
        match message {
            Some(m) => {
                response.data = data;
                response.successful = true;
                response.message = m;
            },
            None => {
                response.data = data;
                response.successful = true;
                response.message = "Request successful.".to_string();
            }
        } 
        response
    }

    /// Creates a new `DetailedResponse` with an error status and the given status code, data, and message
    pub fn consume_error(
        &mut self, 
        status_code: StatusCode, 
        data: Option<T>, 
        message: Option<String>
    ) -> Self {
        let mut response: Self = DetailedResponse::new();
        match message {
            Some(m) =>  {
                match data {
                    Some(d) => response.data = d,
                    None => response.data = T::new(None),
                }

                let mes = format!(
                    "{}: {}", 
                    status_code, 
                    m
                );
                response.message = mes;
            },
            None => {
                match data {
                    Some(d) => response.data = d,
                    None => response.data = T::new(None),
                }
                self.message = "Unknown Error Encountered".to_string();
            }
        }
        response 
    }
}

impl<T: Model + Serialize + Clone> MessageBody for DetailedResponse<T> {
    type Error = Infallible;

    /// Calculates the size of the `DetailedResponse` by serializing it to a string and returning the length
    fn size(&self) -> BodySize {
        // use T's size method to calculate the size of the data field
        BodySize::Sized(serde_json::to_string(self).expect("").len() as u64)
    }

    /// Polls the next chunk of bytes to send in the response body, serializing the
    /// `DetailedResponse` to a string for async requests.
    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {
        let response_data = &self.data;

        let payload_string = serde_json::to_string_pretty(
            & Self {
                data: response_data.clone(),
                successful: self.successful,
                message: self.message.clone(),
            }
        ).unwrap();
        let payload_bytes = Bytes::from(payload_string);

        Poll::Ready(Some(Ok(payload_bytes)))
    }
}
