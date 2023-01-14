use futures::stream::{iter, StreamExt};
use super::cub::Cub;
use std::future::Future;
use serde::Serialize;

type Error = Box::<dyn std::error::Error>;


#[derive(Serialize)]
pub struct ArcticFoxStruct<T: Cub> {
    pub data: T,
    pub message: String,
}

pub enum ArcticFox<T: Cub> {
    Live(T),
    Frozen(T, Error),
}


impl<T: Cub> std::marker::Unpin for ArcticFox< T> {}

impl<T: Cub> std::clone::Clone for ArcticFox<T> {
    fn clone(&self) -> Self {
        match self {
            Live(t)         => Live(t.clone()),
            Frozen(t, fr)   => Frozen(t.clone(), *fr),
        }
    }
}

impl<T: Cub + Serialize > ArcticFox<T> {

    pub fn run<F>(&mut self, f: F) -> &mut Self 
        where F: FnOnce(T) -> Result<T, Error>
    {
        match self {
            ArcticFox::Live(data) => {
                match f(data.clone()) {
                    Ok(new_data)    => { *self = Live(new_data);},
                    Err(fr)         => { *self = Frozen(data.clone(), fr); }
                }
            },
            ArcticFox::Frozen(_, _) => (),
        }
        self
    }

    pub async fn arun<F, Fut>(&mut self, f: F) -> ArcticFox<T>
        where F: FnOnce(T) -> Fut, Fut: Future<Output = Result<T, Error>>
    {
        match self {
            Live(data) => {
                let new_data = f(data.clone()).await;
                match new_data {
                    Ok(new_data)    => { *self = Live(new_data); },
                    Err(fr)         => { *self = Frozen(data.clone(), fr); }
                }
            },
            Frozen(_,_) => (),
        }
        self.clone()
    }

    pub fn freeze(&mut self, freezer: Error) -> T {
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

#[derive(Default, Clone, PartialEq, Serialize)]
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
    I: Iterator<Item = T> + Cub,
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

impl<'a, I, T: 'a> ArcticFox<I>
where
    I: Iterator<Item = &'a T> + Cub + Serialize,
    T: Cub + Serialize + Eq
{
    pub fn new(pack: I) -> ArcticFox<I> {
         Live(pack) 
    }

    /// should not plan to add in any new data here... which i think is obvious.
    pub fn pack<F>(&mut self, f: F) -> &mut ArcticFox<I> 
        where F: FnMut(T) -> Result<T, Error>
    {
        match *self {
            Frozen(_, _) => {},
            Live(s) => {
                let ns: Vec<&'a T> = Vec::from_iter(s);
                let iter = s.enumerate();
                iter.for_each(|(i, mut item)| {
                    match f(*item) {
                        Ok(n_item) => { ns[i] = &n_item; },
                        Err(e) => { 
                            *self = Frozen(s.clone(), e)  
                        },
                    } 
                });
                if let Live(ss) = *self {
                    *self = Live(
                        Pack::new(ss).it
                        )
                } 
            },     
        }
        self
    }

    pub async fn apack<F, Fut>(&mut self, f: F) -> ArcticFox<I>
        where F: FnMut(&'a T) -> Fut, Fut: Future<Output = Result<T, Error>>
    {
        match self {
            Frozen(_, _) => (),
            Live(s) => {
                let erred = false;
                let vector: Vec<&'a T> = Vec::new();
                let stream = iter(s);
                stream.for_each_concurrent(
                    None,
                    async |item| {
                        let _ = match f(item).await {
                            Ok(_) => todo!(),
                            Err(_) => todo!()
                        };
                    });
            }
        }
        *self
    }

    pub fn fpack(&mut self, freezer: Error) -> I {
        match std::mem::replace(self, Frozen(self, freezer)) {
            Frozen(_,_) => {},
            Live(iter) => iter
        }
    }
}

pub use ArcticFox::{Live, Frozen};
