use crate::helpers::{TestApp, Verify2FARequest};


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