#[cfg(test)]
mod tests {
    use actix_web::{
        test,
        web,
        App,
        HttpResponse,
        HttpRequest,
    };
    use tavern_common::{
        data_structures::respondent::{
            Respondent,
            Successful,
            Unsuccessful
        },
        authentication::token::Token,
        tavern_error::TRes
    };

    static USERNAME0: &str = "adamkali";
    static PASSWORD0: &str = "T@v11235";
    static PASSWORD1: &str = "T$v11235";
    static PASSWORD2: &str = "T@v11135";
    static PASSWORD3: &str = "T";
    static PASSWORD4: &str = "T@v 1235";
    static PASSWORD5: &str = "T@v11235813213455891442333776109871597";
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
            Successful(t) => todo!(),
            Unsuccessful(t, e) => todo!()
        }
    }

    #[test]
    pub async fn test_data() {
        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD0);

        let res = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD0.to_string(), 
            EMAILADDR.to_string());

        assert!(res.is_ok())
    }

    #[test]
    pub async fn test_data_that_should_fail() {
        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD1);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD1.to_string(), 
            EMAILADDR.to_string());

        println!("test_data1 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD2);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD2.to_string(), 
            EMAILADDR.to_string());

        println!("test_data2 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD3);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD3.to_string(), 
            EMAILADDR.to_string());

        println!("test_data3 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD4);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD4.to_string(), 
            EMAILADDR.to_string());

        println!("test_data4 result: {:?}", res);

        assert!(matches!(res, Err(_e)));

        let auth_token: Token 
            = Token::default();

        println!("\nThis is the password {}\n", PASSWORD5);

        let res: TRes<Token> = auth_token.init(
            USERNAME0.to_string(), 
            PASSWORD5.to_string(), 
            EMAILADDR.to_string());

        println!("test_data5 result: {:?}", res);

        assert!(matches!(res, Err(_e)));
    }

    #[test]
    pub async fn test_respondent() {
        let mut response: Respondent<Token> =
            Respondent::Successful(Token::default());

        response.run(|token| {
            token.init(
                USERNAME0.to_string(), 
                PASSWORD0.to_string(), 
                EMAILADDR.to_string())
            });

        println!("Is the response successful: {}", response.is_success());
        assert!(response.is_success());

        response = Respondent::Successful(Token::default());

        response.run(|token| {
            token.init(
                USERNAME0.to_string(), 
                PASSWORD1.to_string(), 
                EMAILADDR.to_string())
            });

        println!("Is the response successful: {}", response.is_success());
        assert!(response.not_success());
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
