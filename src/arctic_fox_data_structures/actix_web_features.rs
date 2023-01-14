use core::task::Poll;

pub use crate::{prelude::*, ArcticFox, ArcticFoxStruct};
pub use actix_web::{body::MessageBody, web::Bytes};
pub use serde::Serialize;

static SUCCESSUL: &str = "The Request was successful";

impl<T: Cub + Serialize> MessageBody for ArcticFox<T> {
    type Error = &'static str;

    fn size(&self) -> actix_web::body::BodySize {
        match self {
            Live(t) => actix_web::body::BodySize::Sized(t.size() + SUCCESSUL.len() as u64),
            Frozen(t, e) => {
                actix_web::body::BodySize::Sized(t.size() + (*e).to_string().len() as u64)
            }
        }
    }

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<actix_web::web::Bytes, Self::Error>>> {
        match self.get_mut() {
            Frozen(t, e)    => { 
                match serde_json::to_string_pretty(&ArcticFoxStruct{
                    data:       t.clone(),
                    message:    e.to_string()
                }) {
                    Ok(st) => Poll::Ready(
                        Some(Ok(Bytes::from(st)))
                        ),
                    Err(e) => Poll::Ready(
                        Some(Ok(Bytes::from(e.to_string())))
                        ),
                }
            },
            Live(t)         => { 
                match serde_json::to_string_pretty(&ArcticFoxStruct{
                    data:       t.clone(),
                    message:    SUCCESSUL.to_string()
                }) {
                    Ok(st) => Poll::Ready(
                        Some(Ok(Bytes::from(st)))
                        ),
                    Err(e) => Poll::Ready(
                        Some(Ok(Bytes::from(e.to_string())))
                        ),
                }
            },
        }
    }
}
