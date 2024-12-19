// User management system with error handling and tests
#[derive(Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub active: bool,
}

#[derive(Debug)]
pub enum UserError {
    DuplicateUsername,
    InvalidEmail,
    UserNotFound,
}

pub struct UserManager {
    users: Vec<User>,
    next_id: u32,
}

impl UserManager {
    pub fn new() -> UserManager {
        UserManager {
            users: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_user(&mut self, username: &str, email: &str) -> Result<&User, UserError> {
        // Check for duplicate username
        if self.users.iter().any(|u| u.username == username) {
            return Err(UserError::DuplicateUsername);
        }

        // Validate email
        if !email.contains('@') {
            return Err(UserError::InvalidEmail);
        }

        // Create new user
        let user = User {
            id: self.next_id,
            username: username.to_string(),
            email: email.to_string(),
            active: true,
        };

        self.next_id += 1;
        self.users.push(user);
        Ok(self.users.last().unwrap())
    }

    pub fn deactivate_user(&mut self, username: &str) -> Result<(), UserError> {
        if let Some(user) = self.users.iter_mut().find(|u| u.username == username) {
            user.active = false;
            Ok(())
        } else {
            Err(UserError::UserNotFound)
        }
    }

    // Get all active users
    pub fn get_active_users(&self) -> Vec<&User> {
        self.users.iter().filter(|u| u.active).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for tests
    fn create_test_user() -> (UserManager, &'static str, &'static str) {
        (UserManager::new(), "testuser", "test@example.com")
    }

    #[test]
    fn test_create_user_success() {
        let (mut manager, username, email) = create_test_user();
        let result = manager.create_user(username, email);
        assert!(result.is_ok());

        let user = result.unwrap();
        assert_eq!(user.username, username);
        assert_eq!(user.email, email);
        assert!(user.active);
    }

    #[test]
    fn test_duplicate_username() {
        let (mut manager, username, email) = create_test_user();
        // Create first user
        manager.create_user(username, email).unwrap();
        // Try to create duplicate
        let result = manager.create_user(username, "other@example.com");
        assert!(matches!(result, Err(UserError::DuplicateUsername)));
    }

    #[test]
    fn test_invalid_email() {
        let (mut manager, username, _) = create_test_user();
        let result = manager.create_user(username, "invalid-email");
        assert!(matches!(result, Err(UserError::InvalidEmail)));
    }

    #[test]
    fn test_deactivate_user() {
        let (mut manager, username, email) = create_test_user();
        // Create and deactivate user
        manager.create_user(username, email).unwrap();
        assert!(manager.deactivate_user(username).is_ok());

        // Verify user is inactive
        let active_users = manager.get_active_users();
        assert!(active_users.is_empty());
    }
}
