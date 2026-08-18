use std::{cell::RefCell, path::{PathBuf}};

// Se maneja la variable de entorno por sesión/hilo
// Tener en cuenta, en el ui se tiene un hilo principal
// al abrir un subhilo no se hereda el entorno, pero ,
// en producción no importa ya que ese hilo es None
thread_local! {
    static CONFIG_PREFIX: RefCell<String> = RefCell::new(String::new());
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