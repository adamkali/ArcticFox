use super::cub::Cub;

use std::marker::PhantomData;

#[derive(Default, Clone)]
pub struct AdoptedCub<T> {
    data: T,
    phantom: PhantomData<T>,
}

impl<T: Clone> AdoptedCub<T> {
   pub fn unstore(&self) -> T { self.data.clone() }
}

impl<T: Default + Clone> Cub for AdoptedCub<T> {
    fn size(&self) -> u64 {
        std::mem::size_of::<T>() as u64
    }
}


