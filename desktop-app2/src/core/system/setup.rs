use std::{fs, path::Path};

use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setup{
    pub configured: bool,
    pub central_host: String,
    pub central_port: u16,
}


impl Default for Setup {
    fn default() -> Self {
        Self {
            configured: false,
            central_host: "0".to_string(),
            central_port: 0,
        }
    }
}

pub fn load_config() -> Setup {
    load_config_from(Path::new("config.toml"))
}

fn load_config_from(path: &Path) -> Setup {
    let path = Path::new(path);

    if path.exists() {
        let content = fs::read_to_string(path).unwrap_or_default();
        let table: Table = content.parse().unwrap_or_default();
        Setup {
            configured: table.get("configured").and_then(|v| v.as_bool()).unwrap_or(false),
            central_host: table.get("central_host").and_then(|v| v.as_str()).unwrap_or("0").to_string(),
            central_port: table.get("central_port").and_then(|v| v.as_integer()).unwrap_or(0) as u16
        }
    } else {
        let default = Setup::default();
        save_config_to(&default, path);
        default
    }

}

pub fn save_config(config: &Setup) {
    save_config_to(config, Path::new("config.toml"));
}

fn save_config_to(config: &Setup, path: &Path) {
    let toml_string = toml::to_string_pretty(config).unwrap();
    fs::write(path, toml_string).ok();
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

use super::*;

     #[test]
    fn config_created_if_not_exists() {
        let path = PathBuf::from("/tmp/test_config.toml");
        let _ = fs::remove_file(&path); 
        let config = load_config_from(&path);

        assert_eq!(config.central_host, "0".to_string());
        assert_eq!(config.central_port, 0);
        assert!(path.exists());

        let _ = fs::remove_file(&path);
    }

     #[test]
    fn config_loaded_if_exists() {

        let path = PathBuf::from("/tmp/test_config2.toml");
        let _ = fs::remove_file(&path); 
        let setup= Setup{
            configured: true,
            central_host: "0.0.0.0".to_string(),
            central_port: 80,
        };
        save_config_to(&setup, &path);

        let config = load_config_from(&path);

        assert_eq!(config.central_host, "0.0.0.0");
        assert_eq!(config.central_port, 80);

        let _ = fs::remove_file(&path);
    }
}