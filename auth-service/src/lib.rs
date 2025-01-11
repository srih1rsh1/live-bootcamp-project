use axum::{http::StatusCode, response::IntoResponse, routing::post, serve::Serve, Router};
use reqwest::Url;
use std::error::Error as error;
use tower_http::services::ServeDir;



pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn error>> {

        let router = Router::new().nest_service("/", ServeDir::new("assets"))
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/verify-2fa", post(verify_2fa))
        .route("/logout",post(logout))
        .route("/verify-token", post(verify_token));


        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();


        //Creating a Server
        let server = axum::serve(listener, router);

        //Instance
        let application = Application { server, address };
        
        // Returns
        Ok(application)
    }

    pub async fn run(self: Self) -> Result<(), std::io::Error>{
        println!("listening on {}", &self.address);
        self.server.await
    }
}

async fn signup() -> impl IntoResponse {
    let test = reqwest::Request::new(reqwest::Method::GET, Url::parse("http://localhost").unwrap());
    StatusCode::OK.into_response()
}
async fn login() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn verify_2fa() -> impl IntoResponse {
    StatusCode::OK.into_response()
}
async fn logout() -> impl IntoResponse {
    StatusCode::OK.into_response()
} 
async fn verify_token() -> impl IntoResponse{
    StatusCode::OK.into_response()
}