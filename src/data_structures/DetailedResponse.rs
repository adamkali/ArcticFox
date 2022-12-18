use super::Model::Model;

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

/// `DetailedResponse` => An intelligent communicater between
///     apis, clients, and services.
///
/// `DetailedResponse<T>` is uses the [`Model`] trait to standardize
///     the data propagated through communications by implementing
///     [`actix_web::MessageBody`]
///
/// **Example**: 
/// ```rust
/// use tavern_common::{DetailedResponse, Model}
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
///     // HttpResponse::Ok().body(response.consume_error(StatusCode::NOT_FOUND, "This is a bad error.")
/// }
/// ...
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct DetailedResponse<`a, T: Model + Serialize + Deserialize<`a>> {
    pub data: Option<T>,
    pub successful: bool,
    pub message: Option<String>,
}

impl<T: Model + Serialize + Deserialize> DetailedResponse<T> {
    pub fn new(&self) -> Self {
        return Self {
            data: None,
            successful: false,
            message: None,
        }
    }

    pub fn ok(&self, data: T, message: Option<String>) -> Self {
       match message {
            Some(m) => {
                self.data = Some(data);
                self.message = Some(m);
            },
            None => {
                self.data = Some(data);
                self.message = Some("Response Ok.".to_string());
            }
       } 
       *self
    }

    pub fn consume_error(&self, status_code: StatusCode, message: Option<String>) -> Self {
        match message {
            Some(m) =>  {
                self.data = None;

                let mes = format!(
                    "{}: {}", 
                    status_code.to_string(), 
                    message.unwrap()
                );
                self.message = Some(mes);
            },
            None => {
                self.data = None;
                self.message = Some("Unknown Error Encountered".to_string());
            }
        }
        *self
    }
}

impl<T: Model + Serialize + Deserialize> MessageBody for DetailedResponse<T> {
    type Error = Infallible;

    fn size(&self) -> BodySize {
        // use T's size method to calculate the size of the data field
        match self.data {
            Some(d) => {
                let size = d.size();
                BodySize::Sized(size as u64)
            },
            None => BodySize::Sized(0 as u64)
        } 
    }

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Self::Error>>> {

        let payload_placeholder = Self {
            data: self.data,
            successful: self.successful,
            message: self.message
        };

        let payload_string = serde_json::to_string_pretty(
            &payload_placeholder
        ).unwrap().to_string();
        let payload_bytes = Bytes::from(payload_string);

        Poll::Ready(Some(Ok(payload_bytes)))
    }
}
