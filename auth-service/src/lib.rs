use axum::{Router,serve::Serve};
use std::error::Error as error;
use tower_http::services::ServeDir;


pub struct Application {
    server: Serve<Router, Router>,
    pub address: String,
}

impl Application {
    pub async fn build(address: &str) -> Result<Self, Box<dyn error>> {

        let router = Router::new().nest_service("/", ServeDir::new("assets"));
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
