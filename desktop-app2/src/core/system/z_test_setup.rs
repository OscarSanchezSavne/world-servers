#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};
    use crate::core::system::setup::*;

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

     #[test]
    fn test_init_config() 
    {
        init_config(Some("test_init_config".into()));
        assert_eq!(true, PathBuf::from(format!(".config_test_init_config")).exists());
        clean_config();
    }

     #[test]
    fn test_config_path() 
    {
        self::init_config(Some("test_config_path".into()));

        assert_eq!(
            config_path("file.toml".into()), 
            PathBuf::from(format!(".config_test_config_path/file.toml"))
        );
        self::clean_config();
    }

     #[test]
    fn test_clean_config() 
    {
        self::init_config(Some("test_clean_config".into()));
        assert_eq!(true, PathBuf::from(format!(".config_test_clean_config")).exists());
        self::clean_config();
        assert_eq!(false, PathBuf::from(format!(".config_test_clean_config")).exists());
    }

     #[test]
    fn test_get_config_prefix() 
    {
        self::set_config_prefix("valor1".into());
        assert_eq!(
            "valor1", 
            self::get_config_prefix()
        );
        
    }

}