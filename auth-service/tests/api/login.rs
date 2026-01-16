use crate::helpers::{TestApp, LoginRequest};


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