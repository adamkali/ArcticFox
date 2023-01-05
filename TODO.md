# TavernCommon

## Things to do

- [ ] add the following to `tavern_error.rs`
    - `UnathorizedError(m)`
    - `ForbiddenError(m)`
    - `UserError(m)`
    - `ServerError(m)`
    - `UncommonError(m, sc)`
- [ ] add `pub fn to_code() -> actix_web::http::StatusCode` to tavern_error impl
- [ ] add `pub async fn run_async(&self, f: F) -> Future<Result<(), TavernError>>` to `Respondent`'s implementation.
- [ ] add `respondent_bind!(dyn &T: Model)` to macros that will return a `Respondent::Successful(Model)`

