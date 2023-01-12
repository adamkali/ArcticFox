#[cfg(tests)]
mod tests {

    pub use actix_web;
    pub use arctic_fox::ArcticFox;
    pub use std::marker::PhantomData;

    pub struct Dumb {
        pub phantom: PhantomData<String>,
    }

    

    #[actix_web::test] 
    pub async fn test_single_ArcticFox() {
        
    } 

    #[actix_web::test] 
    pub async fn test_vector_ArcticFox() {
        
    } 
}
