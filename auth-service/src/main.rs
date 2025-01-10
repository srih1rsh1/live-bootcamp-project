use auth_service::Application;

#[tokio::main]
async fn main() {

    let app = Application::build("0.0.0.0:3000").await.expect("Failed to Build app");
    app.run().await.expect("Unable to run the application");
    
}
