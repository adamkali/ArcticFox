use std::{
    default::Default,
    clone::Clone
};

pub trait Cub: Default + Clone + PartialEq {
    fn size(&self) -> u64;
}

