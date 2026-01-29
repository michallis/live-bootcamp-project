use serde::Deserialize;
use validator::ValidateEmail;
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn parse(mail: String) -> Result<Self, String> {
        if mail.validate_email() { Ok(Self(mail)) }
        else { Err(format!("{} is not a valid email.", mail)) }
    }
}

// Convert a reference of one type to a reference of another type
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::Email;

    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::Gen;
    use rand::SeedableRng;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "wrongmail.com".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domain.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary(g: &mut Gen) -> Self {
            let seed: u64 = g.size() as u64;
            let mut rng = rand::rngs::SmallRng::seed_from_u64(seed);
            let email = SafeEmail().fake_with_rng(&mut rng);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }
}