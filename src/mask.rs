pub fn should_mask(key: &str) -> bool {
    let lower = key.to_lowercase();
    lower.contains("key")
        || lower.contains("secret")
        || lower.contains("password")
        || lower.contains("pwd")
        || lower.contains("token")
        || lower.contains("auth")
        || lower.contains("pass")
        || lower.contains("private")
        || lower.contains("cred")
}

pub fn mask_value(val: &str) -> String {
    if val.is_empty() {
        return String::new();
    }
    if val.len() <= 6 {
        return "********".to_string();
    }
    format!("{}********{}", &val[..2], &val[val.len() - 2..])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_mask() {
        assert!(should_mask("AWS_SECRET_ACCESS_KEY"));
        assert!(should_mask("DB_PASSWORD"));
        assert!(should_mask("AUTH_TOKEN"));
        assert!(should_mask("API_CREDENTIALS"));
        assert!(!should_mask("DATABASE_URL"));
        assert!(!should_mask("PORT"));
    }

    #[test]
    fn test_mask_value() {
        assert_eq!(mask_value(""), "");
        assert_eq!(mask_value("short"), "********");
        assert_eq!(mask_value("super_long_secret_value"), "su********ue");
    }
}
