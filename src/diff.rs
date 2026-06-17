use std::collections::BTreeMap;
use colored::*;

pub fn print_diff(env1: &BTreeMap<String, String>, env2: &BTreeMap<String, String>, mask: bool) {
    let mut all_keys = std::collections::BTreeSet::new();
    for k in env1.keys() {
        all_keys.insert(k);
    }
    for k in env2.keys() {
        all_keys.insert(k);
    }

    let mut has_changes = false;

    for key in all_keys {
        let val1 = env1.get(key);
        let val2 = env2.get(key);

        match (val1, val2) {
            (Some(v1), Some(v2)) => {
                if v1 != v2 {
                    has_changes = true;
                    let display_v1 = if mask && crate::mask::should_mask(key) {
                        crate::mask::mask_value(v1)
                    } else {
                        v1.clone()
                    };
                    let display_v2 = if mask && crate::mask::should_mask(key) {
                        crate::mask::mask_value(v2)
                    } else {
                        v2.clone()
                    };
                    println!(
                        "{} {}: {} -> {}",
                        "~".yellow().bold(),
                        key.bold(),
                        display_v1.red(),
                        display_v2.green()
                    );
                }
            }
            (None, Some(v2)) => {
                has_changes = true;
                let display_v2 = if mask && crate::mask::should_mask(key) {
                    crate::mask::mask_value(v2)
                } else {
                    v2.clone()
                };
                println!("{} {}: {}", "+".green().bold(), key.bold(), display_v2.green());
            }
            (Some(v1), None) => {
                has_changes = true;
                let display_v1 = if mask && crate::mask::should_mask(key) {
                    crate::mask::mask_value(v1)
                } else {
                    v1.clone()
                };
                println!("{} {}: {}", "-".red().bold(), key.bold(), display_v1.red());
            }
            (None, None) => {}
        }
    }

    if !has_changes {
        println!("{}", "No differences found between environments.".green());
    }
}
