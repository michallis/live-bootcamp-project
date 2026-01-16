use crate::helpers::{TestApp};


#[tokio::test]
async fn logout_returns_200() {
    let app = TestApp::new().await;
    let token = "some_valid_token";
    let response = app.post_logout(token).await;
    assert_eq!(response.status().as_u16(), 200);
}