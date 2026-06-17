use std::collections::BTreeMap;
use colored::*;

pub fn export_variables(variables: &BTreeMap<String, String>, format: &str, mask: bool) {
    match format {
        "shell" => {
            for (k, v) in variables {
                let display_v = if mask && crate::mask::should_mask(k) {
                    crate::mask::mask_value(v)
                } else {
                    v.clone()
                };
                println!("export {}={}", k, format_shell_val(&display_v));
            }
        }
        "docker" => {
            println!("environment:");
            for (k, v) in variables {
                let display_v = if mask && crate::mask::should_mask(k) {
                    crate::mask::mask_value(v)
                } else {
                    v.clone()
                };
                println!("  - {}={}", k, display_v);
            }
        }
        "json" => {
            let mut masked_vars = BTreeMap::new();
            for (k, v) in variables {
                let display_v = if mask && crate::mask::should_mask(k) {
                    crate::mask::mask_value(v)
                } else {
                    v.clone()
                };
                masked_vars.insert(k.clone(), display_v);
            }
            if let Ok(json_str) = serde_json::to_string_pretty(&masked_vars) {
                println!("{}", json_str);
            }
        }
        _ => {
            for (k, v) in variables {
                let display_v = if mask && crate::mask::should_mask(k) {
                    crate::mask::mask_value(v)
                } else {
                    v.clone()
                };
                println!("{}{} {}", k.cyan().bold(), "=".white(), display_v);
            }
        }
    }
}

fn format_shell_val(val: &str) -> String {
    if val.contains(' ') || val.contains('\"') || val.contains('\'') || val.contains('$') {
        format!("\"{}\"", val.replace('"', "\\\"").replace('$', "\\$"))
    } else {
        val.to_string()
    }
}
