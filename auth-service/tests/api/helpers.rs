use auth_service::app_state::BannedTokenStoreType;
use auth_service::services::HashsetBannedTokenStore;
use auth_service::{
    app_state::AppState, services::HashmapUserStore, utils::constants::test, Application,
};

use reqwest;
use reqwest::cookie::Jar;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub cookie_jar: Arc<Jar>,
    pub http_client: reqwest::Client,
    pub banned_token_store: BannedTokenStoreType,
}

impl TestApp {
    pub async fn new() -> Self {
        let user_store = HashmapUserStore {
            users: HashMap::new(),
        };

        let banned_token_store = HashsetBannedTokenStore {
            token: HashSet::new(),
        };

        let cookie_jar = Arc::new(Jar::default());
        let banned_token_store_type = Arc::new(RwLock::new(banned_token_store));
        let app_store = AppState::new(
            Arc::new(RwLock::new(user_store)),
            banned_token_store_type.clone(),
        );
        let app = Application::build(app_store, test::APP_ADDRESS)
            .await
            .expect("Failed to Build the application");

        let address = format!("http://{}", app.address.clone());

        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::builder()
            .cookie_provider(cookie_jar.clone())
            .build()
            .unwrap();

        let application = TestApp {
            address,
            cookie_jar,
            http_client,
            banned_token_store: banned_token_store_type,
        };

        application
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn signup<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to perform signup")
    }

    pub async fn login<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to login")
    }

    pub async fn logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to logout")
    }

    pub async fn verify_2fa(self: &Self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .send()
            .await
            .expect("Failed to perform two factor auth")
    }

    pub async fn verify_token<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: Serialize,
    {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to verify the token")
    }
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
