use auth_service::{
    app_state::AppState, services::HashmapUserStore, utils::constants::prod, Application,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let user_store = HashmapUserStore {
        users: HashMap::new(),
    };
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to Build app");
    app.run().await.expect("Unable to run the application");
}
