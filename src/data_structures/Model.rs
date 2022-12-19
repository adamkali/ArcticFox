/// A Trait used for the entire backbone of
///          the tavern ecosystem.
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
/// struct DumbModel {
///     username: String,
///     otherthing: String,
/// }
///
/// impl Model for DumbModel {
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
pub trait Model {
    fn new(id: Option<String>) -> Self;
    fn size(&self) -> u64;
}
