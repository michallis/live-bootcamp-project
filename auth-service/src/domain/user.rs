use serde::Deserialize;
use crate::domain::email::Email;
use crate::domain::Password;

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        self.email == other.email
    }
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self { email, password, requires_2fa }
    }
}