use std::error::Error;
use axum::routing::post;
use axum::{Router, serve::Serve, ServiceExt};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

use crate::routes::{login, logout, signup, verify_2fa};

pub mod routes;
mod domain;
mod services;
mod app_state;

pub use services::hashmap_user_store::HashmapUserStore;
pub use domain::User;
pub use app_state::AppState;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let serve_dir_assets = ServeDir::new("assets");
        let router = Router::new()
            //.nest_service("/", serve_dir_assets.clone())
            .route("/signup", post(signup))
            .route("/login", post(login))
            .route("/verify-2fa", post(verify_2fa))
            .route("/logout", post(logout))
            .route("/verify-token", post(verify_2fa))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application { server, address })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}


