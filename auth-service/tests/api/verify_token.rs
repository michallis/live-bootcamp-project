use crate::helpers::{TestApp};


#[tokio::test]
async fn verify_token_returns_200() {
    let app = TestApp::new().await;
    let token = "some_valid_token";
    let response = app.post_verify_token(token).await;
    assert_eq!(response.status().as_u16(), 200);
}