use super::cub::Cub;
use std::future::Future;
use serde::Serialize;


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

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Pack<I> {
    it: I
}

impl<'a, I, T: 'a> Iterator for Pack<I>
where
    I: Iterator<Item = &'a T>,
    T: Cub + Serialize
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
         self.it.next().cloned()
    }
}

impl<'a, I, T: 'a> Pack<I>
where
    I: Iterator<Item = &'a T>,
    T: Cub + Serialize + PartialEq + Eq
{
    pub fn new(it: I) -> Pack<I> {
        Pack { it }
    }

    pub fn operate(
        &mut self, 
        f: &'a mut impl FnMut(T) 
            -> Result<T, Box::<dyn std::error::Error>>
    ) -> Result<(), Box::<dyn std::error::Error>> { 
        while let Some(i) = self.next() {
            if let Err(e) = f(i) {
                return Err(e);
            }
        }
        Ok(())
    }
}
impl<T, I> Cub for Pack<I>
where
    I: Iterator<Item = T> + Default + Clone,
    T: Cub
{
    fn size(&self) -> u64 {
        let mut size: u64 = 0_u64;
        if let Some(i) = self.it.next() {
            let temp = i.size();
            size += temp;
            while let Some(j) = self.it.next() { size += temp; }
        }
        size
    }
}

impl<'a, I, T: 'a> ArcticFox<'a, I>
where
    I: Iterator<Item = &'a T> + Cub,
    T: Cub + Serialize + PartialEq + Eq
{
    pub fn new(pack: I) -> ArcticFox<'a, I> {
         Live(pack) 
    }


}


#[cfg(arctic_actix)]
pub mod arctic_actix {
    use crate::{ArcticFox, prelude::*, Freezer, ArcticFoxStruct};

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

    impl<'a, T: Cub + Serialize + Clone> MessageBody for ArcticFox<'a, T> {
        type Error = Infallible;

        fn size(&self) -> BodySize {

            let mut payload_bytes: u64 = 0_u64;

            match &self {
                Self::Live(data) => {
                    payload_bytes += data.size() + SUCCESSFUL_MESSAGE.len() as u64;
                },
                Self::Frozen(data, freezer) => {
                    payload_bytes += data.size() + freezer.agent().len() as u64;
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
