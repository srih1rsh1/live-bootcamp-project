use crate::domain::Parse;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Password(String);

impl Parse<Password> for  Password {
    fn parse(p: String) -> Result<Password, String> {
        if p.chars().count() < 8 {
            Err("Please Provide the password with mini 8 characters long".to_owned())
        } else {
            Ok(Self(p))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use crate::domain::{email::Parse, password::Password};
    use quickcheck;
    use quickcheck_macros;
    use fake::{faker::internet::en::Password as FakePassword, Fake};

    #[test]
    fn test_password_empty() {
        let password ="".to_string();
        assert!(Password::parse(password).is_err())
    }
    
    #[test]
    fn test_password() {
        let password = "1234564".to_string();
        assert!(Password::parse(password).is_err())
    }

    #[derive(Clone, Debug)]
    struct  ValidPassword(pub String);

    impl quickcheck::Arbitrary for   ValidPassword {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let password = FakePassword(8..30).fake_with_rng(g);
            Self(password)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_passwords(valid_password: ValidPassword) -> bool {
        Password::parse(valid_password.0).is_ok()
    }
}
