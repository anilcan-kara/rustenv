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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_env_file() {
        let file = NamedTempFile::new().unwrap();
        let content = "# Comment\nDATABASE_URL=postgres://localhost/db\nSECRET_KEY=\"mysecret\"\nPORT=3000\n";
        fs::write(file.path(), content).unwrap();

        let parsed = parse_env_file(file.path()).unwrap();
        assert_eq!(parsed.len(), 3);
        assert_eq!(parsed.get("DATABASE_URL").unwrap(), "postgres://localhost/db");
        assert_eq!(parsed.get("SECRET_KEY").unwrap(), "mysecret");
        assert_eq!(parsed.get("PORT").unwrap(), "3000");
    }
}
