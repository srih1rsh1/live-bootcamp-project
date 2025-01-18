use auth_service::{app_state::AppState, services::HashmapUserStore, Application};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore {
        users: HashMap::new(),
    };
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to Build app");
    app.run().await.expect("Unable to run the application");
}
