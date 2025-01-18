use crate::domain::User;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

#[derive(Default)]
pub struct HashmapUserStore {
    pub users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        let email = &user.email;

        match self.users.contains_key(&user.email) {
            true => Err(UserStoreError::UserAlreadyExists),
            false => {
                self.users.insert(email.to_owned(), user);
                Ok(())
            }
        }
    }

    pub async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.contains_key(email) {
            true => Ok(self.users.get(email).unwrap().clone()),
            false => Err(UserStoreError::UserNotFound),
        }
    }

    pub async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(value) => match value.password == password {
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
    use uuid::Uuid;
    #[tokio::test]
    async fn test_add_user() {
        let email = format!("{}@example.com", Uuid::new_v4());
        let password = "456hjack678";
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
        let email = format!("{}@example.com", Uuid::new_v4());
        let password = "456hjack678";
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
        let email = format!("{}@example.com", Uuid::new_v4());
        let password = "456hjack678";
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
