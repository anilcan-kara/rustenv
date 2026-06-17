use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

pub fn parse_env_file(path: &Path) -> Result<BTreeMap<String, String>> {
    let mut map = BTreeMap::new();
    if !path.exists() {
        return Ok(map);
    }

    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read .env file: {:?}", path))?;

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(pos) = line.find('=') {
            let key = line[..pos].trim().to_string();
            let mut val = line[pos + 1..].trim().to_string();

            if (val.starts_with('"') && val.ends_with('"')) || (val.starts_with('\'') && val.ends_with('\'')) {
                val = val[1..val.len() - 1].to_string();
            }

            map.insert(key, val);
        }
    }

    Ok(map)
}

pub fn write_env_file(path: &Path, variables: &BTreeMap<String, String>) -> Result<()> {
    let mut content = String::new();
    for (k, v) in variables {
        let formatted_value = if v.contains(' ') || v.contains('\n') || v.contains('#') {
            format!("\"{}\"", v.replace('"', "\\\""))
        } else {
            v.clone()
        };
        content.push_str(&format!("{}={}\n", k, formatted_value));
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write .env file: {:?}", path))?;

    Ok(())
}
