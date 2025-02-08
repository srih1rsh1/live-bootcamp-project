use crate::domain::{Email, Password, User, UserStore, UserStoreError};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<Email, User>,
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
}
