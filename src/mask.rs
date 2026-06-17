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
