#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web,
        App,
        HttpResponse,
        HttpRequest,
    };
    use arctic_fox::{
        data_structures::{
            respondent::{
                Respondent,
                Successful,
                Unsuccessful
            },
            model::Model
        },
        authentication::{
            token::Token,
            encryption::*
        },
        tavern_error::TRes
    };
    use uuid::Uuid;
    use log::debug;

    static USERNAME0: &str = "adamkali";
    static PASSWORD0: &str = "T!v11235";
    static PASSWORD1: &str = "T$v11235";
    static PASSWORD2: &str = "T!v11135";
    static PASSWORD3: &str = "T";
    static PASSWORD4: &str = "T!v 1235";
    static PASSWORD5: &str = "T!v11235813213455891442333776109871597";
    static EMAILADDR: &str = "adamkali@outlook.com";

    async fn respondent_serialization(_req: HttpRequest) -> HttpResponse {
        let response: Respondent<Token> =
            Respondent::Successful(Token::default());
       
        HttpResponse::Ok()
            .body(response)
    }

    // TODO! Add this as the example for the Respondent
    async fn respondent_model_serialization(_req: HttpRequest) -> HttpResponse {
        let mut response: Respondent<Token> =
            Respondent::Successful(Token::default());

        response.run(|token| { 
            token.init(
                USERNAME0.to_string(), 
                PASSWORD0.to_string(), 
                EMAILADDR.to_string()
                    )}); 

        match response {
            Successful(ref _t) => HttpResponse::Ok().body(response),
            Unsuccessful(t, e) => HttpResponse::build((e.clone()).to_status_code()).body(Unsuccessful(t, e))
        }
    }

    async fn respondent_model_serialization1(_req: HttpRequest) -> HttpResponse {
        let mut response: Respondent<Token> =
            Respondent::Successful(Token::default());

        response.run(|token| { 
            token.init(
                USERNAME0.to_string(), 
                PASSWORD1.to_string(), 
                EMAILADDR.to_string()
                    )}); 

        match response {
            Successful(ref _t) => HttpResponse::Ok().body(response),
            Unsuccessful(t, e) => HttpResponse::build((e.clone()).to_status_code()).body(Unsuccessful(t, e))
        }
    }

    #[test]
    pub async fn test_valid_password() {
        
        let mut is_valid 
            = is_valid_password(PASSWORD0).is_ok();
        assert!(is_valid);

        is_valid 
            = is_valid_password(PASSWORD1).is_ok();
        assert!(is_valid);
    }

    #[test]
    pub async fn test_hash() {
        
        let encrypted = 
            argon_encrypt_salt(
                    PASSWORD0.to_string()
                );

        assert!(encrypted.is_ok());

        let hash = encrypted.ok().unwrap();
        let is_equal =
            validate_password(
                    PASSWORD0.to_string(),
                    &hash
                );
    
        debug!("\n{}\n", hash);

        assert!(is_equal.as_ref().unwrap());
    }

    #[test]
    pub async fn test_data() {
        
        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD0);

        auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD0.to_string(), 
            EMAILADDR.to_string()).unwrap_or_default();

        println!("\n{:?}\n", serde_json::to_string_pretty(&auth_token));

        let validated = validate_password(PASSWORD0.to_string(), 
                                          &auth_token.userhash);

        let result: bool;
        match validated {
            Ok(b) => { result = b; }
            Err(e) => {
                result = false;
                println!("{}\n", e);
            }
        }

        assert!(result)
    }

    #[test]
    pub async fn test_data_that_should_fail() {
        
        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD1);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD1.to_string(), 
            EMAILADDR.to_string());

        debug!("test_data1 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD2);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD2.to_string(), 
            EMAILADDR.to_string());

        debug!("test_data2 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD3);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD3.to_string(), 
            EMAILADDR.to_string());

        debug!("test_data3 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD4);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD4.to_string(), 
            EMAILADDR.to_string());

        debug!("test_data4 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        debug!("\nThis is the password {}\n", PASSWORD5);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD5.to_string(), 
            EMAILADDR.to_string());

        debug!("test_data5 result: {:?}", res);

        assert!(matches!(res, Err(_e)));
    }

    #[test]
    pub async fn test_respondent() {
        
        let mut response = Respondent::Successful(
            Token::new(Some(Uuid::nil().to_string())));

        response.run(|token| {
            token.init(
                USERNAME0.to_string(), 
                PASSWORD0.to_string(), 
                EMAILADDR.to_string())
            });


        let mut token_to_test: Token = response.provide();
        assert!(Uuid::parse_str(&token_to_test.id).unwrap().is_nil());
        assert!(validate_password(PASSWORD0.to_string(),
                                  token_to_test
                                    .userhash
                                    .as_str()).unwrap()
                                    );

        assert!(response.is_success());

        let mut response1 = Respondent::Successful(
            Token::new(Some(Uuid::nil().to_string())));

        response1.run(|token| {
            token.init(
                USERNAME0.to_string(), 
                PASSWORD1.to_string(), 
                EMAILADDR.to_string())
            });

        token_to_test = response1.provide();
        assert!(Uuid::parse_str(&token_to_test.id).unwrap().is_nil());
        assert!(validate_password(PASSWORD1.to_string(),
                                  token_to_test
                                    .userhash
                                    .as_str()).unwrap()
                                    );

        assert!(response.is_success());
    }

    #[actix_web::test]
    pub async fn test_message_body() {
        
        test::init_service(
            App::new()
                .route("/", web::get()
                    .to(respondent_serialization)),
        ).await;
    }
}
