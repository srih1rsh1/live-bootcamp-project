use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

pub trait Parse<T> {
    fn parse(input: String) -> Result<T, String>;
}

impl Parse<Self> for Email {
    fn parse(s: String) -> Result<Self, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err("Please Enter a Valid Email Address".to_owned())
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_email_empty() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn test_email_without_symbol() {
        let email = "hello.com".to_owned();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn test_email_without_subject() {
        let email = "@example.com".to_owned();
        assert!(Email::parse(email).is_err());
    }
    #[test]
    fn test_email() {
        let email = "harsha@example.com".to_string();
        assert!(Email::parse(email).is_ok());
    }
}
