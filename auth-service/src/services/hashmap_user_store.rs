use crate::domain::{
    BannedTokenStore, BannedTokenStoreError, Email, Password, User, UserStore, UserStoreError,
};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
}

pub struct HashsetBannedTokenStore {
    pub token: HashSet<String>,
}

#[async_trait]
impl UserStore for HashmapUserStore {
    async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = user.email.clone();

        match self.users.contains_key(&email) {
            true => Err(UserStoreError::UserAlreadyExists),
            false => {
                self.users.insert(email, user);
                Ok(())
            }
        }
    }

    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.contains_key(&email) {
            true => Ok(self.users.get(&email).unwrap().clone()),
            false => Err(UserStoreError::UserNotFound),
        }
    }

    async fn validate_user(
        &self,
        email: &Email,
        password: &Password,
    ) -> Result<(), UserStoreError> {
        match self.users.get(&email) {
            Some(value) => match value.password.eq(password) {
                true => Ok(()),
                false => Err(UserStoreError::InvalidCredentials),
            },
            None => Err(UserStoreError::UserNotFound),
        }
    }
}

#[async_trait]
impl BannedTokenStore for HashsetBannedTokenStore {
    async fn token_store(&mut self, token: String) -> Result<(), BannedTokenStoreError> {
        match self.token.insert(token) {
            true => Ok(()),
            false => Err(BannedTokenStoreError::InvalidToken),
        }
    }
    async fn check_token(&self, token: String) -> Option<BannedTokenStoreError> {
        match self.token.contains(&token) {
            true => Some(BannedTokenStoreError::TokenAlreadyExists),
            false => None,
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::domain::{Email, Parse, Password};
    use uuid::Uuid;
    #[tokio::test]
    async fn test_add_user() {
        let email = Email::parse(format!("{}@example.com", Uuid::new_v4())).unwrap();
        let password = Password::parse("456hjack678".to_owned()).unwrap();
        let user1 = User::new(email, password.to_owned(), true).await;

        let mut users = HashmapUserStore {
            users: HashMap::new(),
        };

        match users.add_user(user1).await {
            Ok(_successful) => println!("Added User Successfully"),
            Err(error) => println!("{:?}", error),
        }
    }
    #[tokio::test]
    async fn test_get_user() {
        let email = Email::parse(format!("{}@example.com", Uuid::new_v4())).unwrap();
        let password = Password::parse("456hjack678".to_owned()).unwrap();
        let user1 = User::new(email, password.to_owned(), true).await;

        let users = HashmapUserStore {
            users: HashMap::new(),
        };

        match users.get_user(&user1.email).await {
            Err(error) => println!("{:?}", error),
            Ok(output) => println!("Found user info: {:?} for email:", output),
        }
    }

    #[tokio::test]
    async fn test_validate_user() {
        let email = Email::parse(format!("{}@example.com", Uuid::new_v4())).unwrap();
        let password = Password::parse("456hjack678".to_owned()).unwrap();
        let user1 = User::new(email, password.to_owned(), true).await;

        let users = HashmapUserStore {
            users: HashMap::new(),
        };

        match users.validate_user(&user1.email, &password).await {
            Err(error) => println!("{:?}", error),
            Ok(output) => println!("Found user info: {:?} for email:", output),
        }
    }

    #[tokio::test]
    async fn test_token_add_to_banned_store() {
        let token = "auR17xKQ8q7cjgUiMGzga8JzTRdYFAUZ7K7FtlGPJyNSRHXimO1g22F7Rf8cdTlhX1fcPfcvRH15v6U975pF7nDU6h0CDSvhI31O".to_owned();

        let mut banned_token_store = HashsetBannedTokenStore {
            token: HashSet::new(),
        };

        for i in 0..2 {
            match banned_token_store.token_store(token.clone()).await {
                Err(error) => println!("{:?}", error),
                Ok(value) => println!("Added the Token to the BannedTokenStore"),
            }
        }
    }

    #[tokio::test]
    async fn test_check_token_in_banaed_store() {
        let token = "auR17xKQ8q7cjgUiMGzga8JzTRdYFAUZ7K7FtlGPJyNSRHXimO1g22F7Rf8cdTlhX1fcPfcvRH15v6U975pF7nDU6h0CDSvhI31O".to_owned();
        let mut banned_token_store = HashsetBannedTokenStore {
            token: HashSet::new(),
        };

        match banned_token_store.token_store(token.clone()).await {
            Err(error) => println!("{:?}", error),
            Ok(value) => println!("Added the Token to the BannedTokenStore"),
        }

        match banned_token_store.check_token(token).await {
            Some(error) => println!("{:?}", error),
            None => (),
        }
    }
}
