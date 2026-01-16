use crate::helpers::{TestApp, SignupRequest, LoginRequest, Verify2FARequest};

// Tokio's test macro is used to run the test in an async environment
#[tokio::test]
async fn root_returns_auth_ui() {
    let app = TestApp::new().await;
    let response = app.get_root().await;

    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_200() {
    let app = TestApp::new().await;
    let signup_request = SignupRequest {
        email: "test@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let response = app.post_signup(signup_request).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn login_returns_200() {
    let app = TestApp::new().await;
    let login_request = LoginRequest {
        email: "test@gmail.com".to_string(),
        password: "testpassword".to_string(),
    };
    let response = app.post_login(login_request).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_returns_200() {
    let app = TestApp::new().await;
    let verify_2fa_request = Verify2FARequest {
        login_attempt_id: "123456".to_string(),
        code_2fa: "111111".to_string(),
    };
    let response = app.post_verify_2fa(verify_2fa_request).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;
    let token = "some_valid_token";
    let response = app.post_logout(token).await;
    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;
    let token = "some_valid_token";
    let response = app.post_verify_token(token).await;
    assert_eq!(response.status().as_u16(), 200);
}

// TODO: Implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token)
// For now, simply assert that each route returns a 200 HTTP status code.