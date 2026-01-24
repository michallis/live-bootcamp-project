use std::collections::HashMap;
use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        if self.users.contains_key(&user.email) { return Err(UserStoreError::UserAlreadyExists); }
        self.users.insert(user.email.clone(), user);
        Ok(())
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
        self.users.get(email).ok_or(UserStoreError::UserNotFound).map(|user|  user.clone())
    }

    pub fn validate_user(&self, email: impl Into<String>, password: impl Into<String>) -> Result<(), UserStoreError> {
        let user = self.users.get(&email.into()).ok_or(UserStoreError::UserNotFound)?;
        if user.password != password.into() { return Err(UserStoreError::InvalidCredentials); }
        Ok(())
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("user_a@example.com", "password123", true);
        match store.add_user(user_a.clone()) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false)
        }
    }

    #[tokio::test]
    #[should_panic(expected = "UserAlreadyExists")]
    async fn test_add_user_fails_when_already_exists() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("user_a@example.com", "password123", true);
        let _ = store.add_user(user_a.clone()); // add first time
        match store.add_user(user_a.clone()) {  // add again
            Ok(_) => {}
            Err(_) => panic!("UserAlreadyExists"),
        }
    }

    #[tokio::test]
    async fn test_get_user() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone());
        let _ = store.get_user("michallis@trust1team.com")
            .map(|user| assert_eq!(user.email, user_a.email))
            .map_err(|_|  assert!(false));
    }

    #[tokio::test]
    async fn test_validate_user() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone());
        let _ = store.validate_user("michallis@trust1team.com", "123456")
            .map_err(|_|  assert!(false));
    }

    #[tokio::test]
    #[should_panic(expected = "InvalidCredentials")]
    async fn test_validate_user_wrong_password() {
        let mut store = HashmapUserStore::default();
        let user_a = User::new("michallis@trust1team.com", "123456", true);
        let _ = store.add_user(user_a.clone());
        match store.validate_user("michallis@trust1team.com", "654321") {
            Ok(_) => assert!(false),
            Err(_) => panic!("InvalidCredentials")
        }
    }
}
