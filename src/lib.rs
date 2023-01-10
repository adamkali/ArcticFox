//! ArcticFox is a freezable functional box 'pointer'. It stores data inside of a struct that
//! implements the `Cub` triat by using the bond macro or by using the adoption macro you can 
//! register a primative struct to an ArcticFox. This will create an AdoptedCub trait for 
//! communication. As long as the operation done in run that is Ok() will be considered Successful
//! and the operation will take effect; however as soon as there is an Unsuccessful(), Arctic fox
//! will freeze the operations and the Cub will stay frozen for the rest of the lifetime of the
//! ArcticFox's life time.
//!
//! # Example
//!
//! ## Assigning and operating on the ArcticFox Monad.
//!
//! ```rust
//!  let fox: ArcticFox = bond!(FooCub::default());
//!
//!  fox.run(|foo| {
//!      println!("{}", foo);
//!  })
//! ```
//! ## Assigning and operating on the `ArcticFox` Monad with an `AdoptedCub`
//!
//! ```rust
//! let fox: ArcticFox = adopt!(false);
//!
//! fox.async_run(|bool_cub| async move {
//!     while bool_cub {
//!         // do some async calls and if they fail update bool_cub
//!     }
//! })
//!
//! ```
mod arctic_fox_data_structures;

#[macro_use]
pub mod macros;

pub mod prelude {
    pub use crate::arctic_fox_data_structures::arctic_fox_monad::{
        Frozen,
        Live,
    };
    pub use crate::arctic_fox_data_structures::cub::Cub;
    pub use crate::arctic_fox_data_structures::adopted_cub::AdoptedCub;
}

pub use crate::arctic_fox_data_structures::arctic_fox_monad::{
    ArcticFox,
    ArcticFoxStruct,
    Freezer
};
