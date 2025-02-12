use crate::domain::{email::Email, password::Password, User};
use async_trait::async_trait;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Debug, PartialEq)]
pub enum BannedTokenStoreError {
    TokenAlreadyExists,
    InvalidToken,
}

#[async_trait]
pub trait UserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError>;

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError>;

    async fn validate_user(&self, email: &Email, password: &Password)
        -> Result<(), UserStoreError>;
}

#[async_trait]
pub trait BannedTokenStore {
    async fn token_store(&mut self, token: String) -> Result<(), BannedTokenStoreError>;

    async fn check_token(&self, token: String) -> Option<BannedTokenStoreError>;
}
