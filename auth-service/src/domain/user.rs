
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub async fn new(email: String, password: String, requires_2fa: bool) -> User {
        User {
            email,
            password,
            requires_2fa,
        }
    } 
}