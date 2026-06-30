use std::{cell::RefCell, fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};
use toml::Table;

// Se maneja la variable de entorno por sesión/hilo
// Tener en cuenta, en el ui se tiene un hilo principal
// al abrir un subhilo no se hereda el entorno, pero ,
// en producción no importa ya que ese hilo es None
thread_local! {
    static CONFIG_PREFIX: RefCell<String> = RefCell::new(String::new());
}

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

pub fn init_config(environment: Option<String>) {
    let prefix = environment
        .map(|p| format!("_{}", p))
        .unwrap_or_default();
    
    self::set_config_prefix(prefix.clone());

    let path = PathBuf::from(format!(".config{}", prefix));

    if !PathBuf::from(&path).exists() {
        std::fs::create_dir_all(&path).expect("Failed to create .config directory");
    }
}

pub fn config_path(file: String)-> PathBuf
{
    let prefix= self::get_config_prefix();
    PathBuf::from(format!(".config{}/{}", prefix, file))
}

#[cfg(test)]
pub fn clean_config() {
    let path = self::config_path("".into());
    if PathBuf::from(&path).exists() {
        std::fs::remove_dir_all(&path).expect("Failed to remove config directory");
    }
}


pub (crate) fn load_config_from(path: &Path) -> Setup {
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

pub (crate) fn save_config_to(config: &Setup, path: &Path) {
    let toml_string = toml::to_string_pretty(config).unwrap();
    fs::write(path, toml_string).ok();
}

pub fn get_config_prefix() -> String {
    CONFIG_PREFIX.with(|cell| {
        cell.borrow().clone()
    })
}

pub fn set_config_prefix(prefix: String) {
    CONFIG_PREFIX.with(|cell| {
        *cell.borrow_mut() = prefix;
    });
}