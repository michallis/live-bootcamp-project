use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};
use crate::domain::AuthAPIError;
use crate::domain::datastores::UserStore;

pub async fn signup(State(state): State<AppState>, Json(request): Json<SignupRequest>) -> Result<impl IntoResponse, AuthAPIError> {

    let email = request.email;
    let password = request.password;

    // Verify input params
    if email.is_empty() || password.chars().count() < 8 { return Err(AuthAPIError::InvalidCredentials) }
    let mut user_store = state.user_store.write().await;

    match user_store.get_user(&email).await {
        Ok(_) => return Err(AuthAPIError::UserAlreadyExists),
        Err(_) => { },
    }

    let user = User::new(email.clone(), password, request.requires_2fa);
    match user_store.add_user(user).await {
        Ok(_) => {
            let response = Json(SignupResponse {
                message: "User created successfully!".to_string(),
            });

            Ok((StatusCode::CREATED, response))
        }
        Err(_) => return Err(AuthAPIError::UnexpectedError),
    }

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