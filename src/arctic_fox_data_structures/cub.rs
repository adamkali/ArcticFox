use std::{
    default::Default,
    clone::Clone
};

/// A Trait used for the entire backbone of
/// Arctic fox 
///
/// Required Functions =>
/// ```
///     fn new(&self, id: Option<String>) -> self;
///     fn size(&self) -> u64;
/// ```
///
/// Used to transfer and manipulate data inide the apis.
///
/// Example => 
///
/// ```
/// struct DumbCub {
///     username: String,
///     otherthing: String,
/// }
///
/// impl Cub for DumbCub {
///     pub fn new(&self, id: Option<String>) -> Self {
///         match id {
///             Some(i) => {
///                 self.username = i;
///                 self.otherthing = "other".to_string();
///             },
///             None => {
///                 self.username = "0000".to_string();
///                 self.otherthing = "other".to_string();
///             }
///         }
///         self
///     }
///
///     pub fn size(&self) -> u64 {
///         let size_of_struct: as u64;
///         size_of_struct += self.username.len() as u64;
///         size_of_struct += self.username.len() as u64;
///     }
/// }
/// ```
pub trait Cub: Default + Clone  {
    fn new(id: Option<String>) -> Self;
    fn size(&self) -> u64;
}


