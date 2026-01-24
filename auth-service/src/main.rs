use std::sync::Arc;
use tokio::sync::RwLock;
use auth_service::{AppState, Application, User};
use auth_service::HashmapUserStore;

#[tokio::main]
async fn main() {
    let mut user_store = HashmapUserStore::default();
    populate_users(&mut user_store).await;
    let app_state = AppState::new(Arc::new(RwLock::new(user_store)));

    let app = Application::build(app_state, "0.0.0.0:3000")
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}

async fn populate_users(store: &mut HashmapUserStore) {
    let user_test = User::new("test@gmail.com", "1234", true);

    let _ = store.add_user(user_test);
}
