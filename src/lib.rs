//! `ArcticFox`  can be summed up as:
//! * Freezing: `ArcticFox` is a data_structure that freezes the mutability of data stored when an error occurs.
//! * Functional: `ArcticFox` is a chainable monad-like structure wich can be used to have execution run in
//! inside a run function.
//! * Freaking close to a Box: `ArcticFox` can be considered a Smart Pointer in the way that it is
//! storing the data and can have its data accessed by `ArcticFox::freeze()` and then can be used
//! in whatever way you want.
//!
//! ## Mindset
//! The proper way to use an ArcticFox should only be used when you dont really care what the error
//! is, just that there was an error and depending on what that error was then you deal with it.
//! Because of this, ArcticFox has Actix directly implemented as a feature. 

mod arctic_fox_data_structures;

#[macro_use]
pub mod macros;

pub mod prelude {
    pub use crate::arctic_fox_data_structures::arctic_fox_monad::{
        Frozen,
        Live,
        Pack
    };
    pub use crate::arctic_fox_data_structures::cub::Cub;
}

pub use crate::arctic_fox_data_structures::arctic_fox_monad::{
    ArcticFox,
    ArcticFoxStruct,
};

