use std::collections::HashMap;
use async_trait::async_trait;
use crate::domain::datastores::{UserStore, UserStoreError};
use crate::domain::User;

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

#[async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) { return Err(UserStoreError::UserAlreadyExists); }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
        self.users.get(email).ok_or(UserStoreError::UserNotFound).map(|user|  user.clone())
    }

    async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
        if user.password != password { return Err(UserStoreError::InvalidCredentials); }
        Ok(())
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("user_a@example.com", "password123", true);
        // Test adding a new user
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user
        let result = user_store.add_user(user).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    #[should_panic(expected = "UserAlreadyExists")]
    async fn test_add_user_fails_when_already_exists() {
        let mut user_store = HashmapUserStore::default();
        let user = User::new("user_a@example.com", "password123", true);
        let _ = user_store.add_user(user.clone()).await; // add first time
        match user_store.add_user(user.clone()).await {  // add again
            Ok(_) => {}
            Err(_) => panic!("UserAlreadyExists"),
        }
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone()).await;
        let _ = store.get_user("michallis@trust1team.com").await
            .map(|user| assert_eq!(user.email, user_a.email))
            .map_err(|_|  assert!(false));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone()).await;
        let _ = store.validate_user("michallis@trust1team.com", "123456").await
            .map_err(|_|  assert!(false));
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidCredentials")]
    async fn test_validate_user_wrong_password() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone()).await;
        match store.validate_user("michallis@trust1team.com", "654321").await {
            Ok(_) => assert!(false),
            Err(_) => panic!("InvalidCredentials")
        }
    }
}
