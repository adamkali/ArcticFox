use actix_web::dev::AppService;
use surrealdb::{
    Datastore,
    Session
};
use crate::tavern_error::TavernError;

type TavernModelResult = Result<T: Model + IntoIterator<Item = T>, TavernError>;

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

/**
 * A trait for implementing routes into the various apis.
 *
 * The `Controller` trait allows for the creation of a struct that can hold routes for a specific resource.
 * It requires that `T` implements the `Model` trait and can be converted into an iterator of itself.
 *
 * # Examples
 *
 * Implementing the `Controller` trait with a custom struct:
 *
 * ```
 * # use std::convert::IntoIterator;
 * # use actix_web::{App, HttpResponse};
 * # use tavernCommon::Model;
 *
 * struct UserController;
 *
 * impl Controller<User> for UserController {
 *     fn new() -> Self {
 *         UserController
 *     }
 *
 *     fn sanitize() {
 *         // some code to sanitize data
 *     }
 *
 *     fn register(self, config: &mut AppService) {
 *         config.service(
 *             web::resource("/api/users")
 *                 .route(web::get().to(|| HttpResponse::Ok()))
 *         );
 *     }
 * }
 *
 * # fn main() {}
 * ```
 *
 * Adding a `Controller` to an `App` instance:
 *
 * ```
 * # use actix_web::{App, HttpResponse};
 * # use TavernCommon::Model;
 * #
 * # struct UserController;
 * #
 * # impl Controller<User> for UserController {
 * #     fn new() -> Self {
 * #         UserController
 * #     }
 * #
 * #     fn sanitize() {
 * #         // some code to sanitize data
 * #     }
 * #
 * #     fn register(self, config: &mut AppService) {
 * #         config.service(
 * #             web::resource("/api/users")
 * #                 .route(web::get().to(|| HttpResponse::Ok()))
 * #         );
 * #     }
 * # }
 *
 * let app = App::new()
 *     .configure(UserController::new().register);
 * ```
 *
 * # Required functions
 *
 * The `Controller` trait requires the following functions to be implemented:
 *
 * - `new() -> Self`: creates a new instance of the implementing struct
 * - `sanitize()`: a function to sanitize data before it is used in the api
 * - `register(self, config: &mut AppService)`: a function to register routes to an `App` instance
 */
pub trait Controller<T: Model + IntoIterator<Item = T>> {
    fn new() -> Self;
    fn sanitize();
    fn register(self, config: &mut AppService); 
}


/**
 * A trait for handeling the neededdata manipulation in Tavern's apis.
 *
 * ## Example 
 *
 * ### Get all for the database:
 * ```
 * async fn get_all_foos(ds: &Datastore, session: &Session ) -> Vec<Foo> {
 *     let ast = parse("USE NS tavern DB tavern_profile; SELECT * FROM foo;")
 *     let res = 
 * }
 * ```
 * 
 */
pub trait Repository<T: Model> {
    fn get_all(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn get(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn create_batch(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn create(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn update_or_insert_batch(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn update_or_insert(ds: &Datastore, session: &Session ) -> TavernModelResult;
    fn delete(ds: &Datastore, session: &Session ) -> TavernModelResult;
}
