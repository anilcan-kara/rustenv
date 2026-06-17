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
