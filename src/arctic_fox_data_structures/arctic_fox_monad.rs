use super::cub::Cub;

//use std::{
//    convert::Infallible, 
//    pin::Pin, 
//    task::Context,
//    future::Future,
//};

use std::future::Future;

use serde::Serialize;

//use crate::{unauthorized, bond};

#[derive(Serialize)]
pub struct ArcticFoxStruct<T: Cub> {
    pub data: T,
    pub message: String,
}

pub trait Freezer {
    fn froze(&self, freezer_reason: &str) -> &dyn Freezer;
    fn agent(&self) -> String;
} 

pub enum ArcticFox<'a, T: Cub> {
    Live(T),
    Frozen(T, &'a dyn Freezer),
}

impl<'a, T: Cub> std::marker::Unpin for ArcticFox<'a, T> {}

impl<'a, T: Cub> std::clone::Clone for ArcticFox<'a, T> {
    fn clone(&self) -> Self {
        match self {
            Live(t) => Live(t.clone()),
            Frozen(t, fr) => Frozen(t.clone(), fr.froze(fr.agent().as_str())),
        }
    }
}

impl<'a, T: Cub + Serialize > ArcticFox<'a, T> {
    pub fn run<F>(&mut self, f: F) -> &mut Self 
        where F: FnOnce(T) -> Result<T, &'a dyn Freezer>
    {
        match self {
            ArcticFox::Live(data) => {
                match f(data.clone()) {
                    Ok(new_data) => {*self = ArcticFox::Live(new_data);},
                    Err(fr) => { let _ = self.freeze(fr); }
                }
            },
            ArcticFox::Frozen(_, _) => (),
        }
        self
    }

    pub async fn async_run<F, Fut>(&mut self, f: F) -> ArcticFox<'a, T>
        where F: FnOnce(T) -> Fut, Fut: Future<Output = Result<T, &'a dyn Freezer>>
    {
        match self {
            Live(data) => {
                let new_data = f(data.clone()).await;
                match new_data {
                    Ok(new_data) => { *self = Live(new_data); },
                    Err(fr) => { let _ = self.freeze(fr); }
                }
            },
            Frozen(_,_) => (),
        }
        self.clone()
    }

    pub fn freeze(&mut self, freezer: &'a dyn Freezer) -> T {
        let holder: T; 
        match self {
            Live(t) => {
                holder = t.clone();
                *self = Frozen(holder.clone(), freezer);
            },
            Frozen(t, _) => { holder = t.clone(); } 
        } 
        holder
    }
}

#[cfg(feature = "arctic_actix")]
pub mod arctic_actix {
    use crate::{ArcticFox, prelude::*, Freezer};

    use core::task::Poll;
    use actix_web::{
        web::Bytes,
        body::{MessageBody, BodySize},
    };

    use std::{
        convert::Infallible, 
        pin::Pin, 
        task::Context,
        future::Future,
    };

    use serde::Serialize;

    static SUCCESSFUL_MESSAGE: &str = "The request was successful";

    impl<T: Cub + Serialize + Clone> MessageBody for ArcticFox<T> {
        type Error = Infallible;

        fn size(&self) -> BodySize {

            let mut payload_bytes: u64 = 0_u64;

            match &self {
                Self::Live(data) => {
                    payload_bytes += data.size() + SUCCESSFUL_MESSAGE.len() as u64;
                },
                Self::Frozen(data, fox_error) => {
                    payload_bytes += data.size() + fox_error.err().len() as u64;
                }
            }

            BodySize::Sized(payload_bytes)
        }

        fn poll_next(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Bytes, Self::Error>>> {
            let data = self.get_mut();

            let payload_string = match data {
                ArcticFox::Live(d) => {
                    serde_json::to_string_pretty(
                        & ArcticFoxStruct {
                            data: d.clone(),
                            message: SUCCESSFUL_MESSAGE.to_string(),
                        }
                    )
                },
                ArcticFox::Frozen(d, e) => {
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
}

pub use ArcticFox::{Live, Frozen};
