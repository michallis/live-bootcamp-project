use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> impl IntoResponse {
    let mut user_store = state.user_store.write().await;
    let _ = user_store.add_user(User::from(request)).unwrap();
    let response = Json(SignupResponse {
        message: "User created successfully!".to_string(),
    });
    (StatusCode::CREATED, response)
}

#[derive(Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl From<SignupRequest> for User {
    fn from(req: SignupRequest) -> Self {
        Self {
            email: req.email,
            password: req.password,
            requires_2fa: req.requires_2fa,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct SignupResponse {
    pub message: String,
}