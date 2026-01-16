use crate::helpers::{TestApp, SignupRequest};


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