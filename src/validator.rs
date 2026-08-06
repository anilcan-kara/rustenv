use std::collections::BTreeMap;
use colored::*;

pub struct ValidationResult {
    pub key: String,
    pub message: String,
    pub is_error: bool,
}

pub fn validate_variables(variables: &BTreeMap<String, String>) -> Vec<ValidationResult> {
    let mut results = Vec::new();

    for (k, v) in variables {
        if v.trim().is_empty() {
            results.push(ValidationResult {
                key: k.clone(),
                message: "Value is empty".to_string(),
                is_error: true,
            });
        }

        let is_valid_key_chars = k.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
        if !is_valid_key_chars {
            results.push(ValidationResult {
                key: k.clone(),
                message: "Key should only contain uppercase letters, digits, and underscores".to_string(),
                is_error: false,
            });
        }

        if k.ends_with("_URL") {
            if !v.starts_with("http://") && !v.starts_with("https://") {
                results.push(ValidationResult {
                    key: k.clone(),
                    message: format!("Value '{}' does not appear to be a valid URL (must start with http:// or https://)", v),
                    is_error: false,
                });
            }
        }

        if k.ends_with("_EMAIL") {
            if !v.contains('@') || !v.contains('.') {
                results.push(ValidationResult {
                    key: k.clone(),
                    message: format!("Value '{}' does not appear to be a valid email address", v),
                    is_error: false,
                });
            }
        }

        if k.ends_with("_PORT") {
            if v.parse::<u16>().is_err() {
                results.push(ValidationResult {
                    key: k.clone(),
                    message: format!("Value '{}' is not a valid port number (must be a number between 0 and 65535)", v),
                    is_error: true,
                });
            }
        }

        if k.ends_with("_BOOL") || k.ends_with("_BOOLEAN") || k.starts_with("ENABLE_") || k.starts_with("DISABLE_") {
            let lower_val = v.to_lowercase();
            if lower_val != "true" && lower_val != "false" && lower_val != "1" && lower_val != "0" {
                results.push(ValidationResult {
                    key: k.clone(),
                    message: format!("Value '{}' is not a valid boolean (must be true/false or 1/0)", v),
                    is_error: false,
                });
            }
        }
    }

    results
}

pub fn print_validation(results: &[ValidationResult]) -> bool {
    if results.is_empty() {
        println!("{}", "All variables validated successfully!".green());
        return true;
    }

    let mut has_errors = false;
    for res in results {
        if res.is_error {
            has_errors = true;
            println!(
                "{} {}: {}",
                "ERROR".red().bold(),
                res.key.bold(),
                res.message
            );
        } else {
            println!(
                "{} {}: {}",
                "WARN".yellow().bold(),
                res.key.bold(),
                res.message
            );
        }
    }

    !has_errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_boolean_warning() {
        let mut map = BTreeMap::new();
        map.insert("ENABLE_FEATURE".to_string(), "invalid_bool".to_string());
        let res = validate_variables(&map);
        assert_eq!(res.len(), 1);
        assert!(!res[0].is_error); // is warning
        assert!(res[0].message.contains("not a valid boolean"));
    }
}
