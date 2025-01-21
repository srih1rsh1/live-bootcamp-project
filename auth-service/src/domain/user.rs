use super::email::Email;
use crate::domain::password::Password;


#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub async fn new(email: Email, password: Password, requires_2fa: bool) -> User {
        User {
            email,
            password,
            requires_2fa,
        }
    } 
}