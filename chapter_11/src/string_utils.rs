// String utility functions with tests
pub struct StringUtils {
    case_sensitive: bool,
}

impl StringUtils {
    pub fn new(case_sensitive: bool) -> StringUtils {
        StringUtils { case_sensitive }
    }

    // Count occurrences of a pattern in text
    pub fn count_occurrences(&self, text: &str, pattern: &str) -> usize {
        if self.case_sensitive {
            text.matches(pattern).count()
        } else {
            text.to_lowercase().matches(&pattern.to_lowercase()).count()
        }
    }

    // Validate email format
    pub fn is_valid_email(&self, email: &str) -> bool {
        // Basic email validation
        email.contains('@')
            && email.split('@').count() == 2
            && !email.starts_with('@')
            && !email.ends_with('@')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive_count() {
        let utils = StringUtils::new(true);
        let text = "The quick brown fox jumps over the lazy dog";
        assert_eq!(utils.count_occurrences(text, "the"), 1);
        assert_eq!(utils.count_occurrences(text, "The"), 1);
    }

    #[test]
    fn test_case_insensitive_count() {
        let utils = StringUtils::new(false);
        let text = "The quick brown fox jumps over the lazy dog";
        assert_eq!(utils.count_occurrences(text, "the"), 2);
    }

    #[test]
    fn test_email_validation() {
        let utils = StringUtils::new(true);
        // Test valid emails
        assert!(utils.is_valid_email("test@example.com"));
        // Test invalid emails
        assert!(!utils.is_valid_email("invalid-email"));
    }
}
