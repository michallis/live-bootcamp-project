use std::sync::Arc;
use auth_service::{AppState, Application, HashmapUserStore, User};
use reqwest::Response;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let mut user_store = HashmapUserStore::default();
        let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

        let app = Application::build(app_state, "127.0.0.1:0")
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        // Run the auth service in a separate async task
        // to avoid blocking the main test thread. 
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        let http_client =reqwest::Client::new();

        // Create new `TestApp` instance and return it
        TestApp { address, http_client }
    }

    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup<Body>(&self, body: &Body) -> Response 
    where 
        Body: serde::Serialize{
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }   

    pub async fn post_login(&self, body: LoginRequest) -> Response {
        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_2fa(&self, body: Verify2FARequest) -> Response {
        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_logout(&self, token: &str) -> Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_verify_token(&self, token: &str) -> Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .bearer_auth(token)
            .send()
            .await
            .expect("Failed to execute request.")
    }
        // TODO: Implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)
}

pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

#[derive(serde::Serialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct Verify2FARequest {
    pub login_attempt_id: String,
    pub code_2fa: String,
}