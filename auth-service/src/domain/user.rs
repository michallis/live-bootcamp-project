use serde::Deserialize;

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Deserialize, Clone, Debug)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl PartialEq<User> for User {
    fn eq(&self, other: &User) -> bool {
        self.email == other.email
    }
}

impl User {
    pub fn new(email: impl Into<String>, password: impl Into<String>, requires_2fa: bool) -> Self {
        Self { email: email.into(), password: password.into(), requires_2fa }
    }
}
