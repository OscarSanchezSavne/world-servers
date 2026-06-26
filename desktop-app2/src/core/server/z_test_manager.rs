#[cfg(test)]
mod tests {
    use std::{fs::{self}, path::PathBuf, str::FromStr};
    use crate::{core::{server::manager::{Server, expand_path}, system::{setup::{self, config_path, init_config}}}, ui::utilities::ExecutionState};

    #[test]
    fn test_expand_tilde_relative() {
        let original_home = std::env::var_os("HOME");
        unsafe{
            std::env::set_var("HOME", "/home/test/");
        }
        let expected_path= expand_path("~/.ssh/key");
        
         if let Some(home) = original_home {
            unsafe{
                std::env::set_var("HOME", home);
            }
        }
        
        assert_eq!(
            expected_path, 
            PathBuf::from_str("/home/test/.ssh/key").unwrap()
        );
    }

    #[test]
    fn test_expand_tilde_absolute() {
        let expected_path= expand_path("/home/.ssh/key");
        assert_eq!(
            expected_path, 
            PathBuf::from_str("/home/.ssh/key").unwrap()
        );
    }

    #[test]
    fn test_validate_returns_first_error_when_multiple_fields_empty() {
        let server = Server::default();
        let err = server.validate().unwrap_err();
        assert_eq!(err, "Server Name is required");
    }

    #[test]
    fn test_validate_ip_invalid_and_user_empty_returns_ip_error() {
        setup::init_config(Some("test_validate_ip_invalid_and_user_empty_returns_ip_error".to_string()));
        let mut server = Server::default();
        server.server_name= "Server".to_string();
        server.server_ip= "invalid-ip".to_string();
        let err = server.validate().unwrap_err();
        assert_eq!(err, "Invalid IP address format");
        setup::clean_config();
    }

    #[test]
    fn test_validate_passphrase_required_when_use_passphrase_is_true() {
        setup::init_config(Some("test_validate_passphrase_required_when_use_passphrase_is_true".to_string()));
        let server = Server {
            server_name: "Ok".to_string(),
            server_ip: "192.168.1.1".to_string(),
            ssh_user: "root".to_string(),
            private_key_path: "/tmp/some_key".to_string(),
            use_passphrase: true,
            passphrase: "  ".to_string(),
            password: "root".into(),
            server_port: 22,
            use_password: false,
        };
        
        fs::remove_file(PathBuf::from_str("/tmp/some_key").unwrap()).ok(); 
        fs::write(&server.private_key_path, "").ok();

        let err = server.validate().unwrap_err();
        assert_eq!(
            err,
            "Passphrase is required when enabled"
        );
        
        fs::remove_file(PathBuf::from_str("/tmp/some_key").unwrap()).ok(); 
        setup::clean_config();
    }

    #[test]
    fn test_save_validated_server() 
    {
        setup::init_config(Some("test_save_validated_server".to_string()));
        let server = Server {
            server_name: "Test".into(),
            server_ip: "127.0.0.1".into(),
            ssh_user: "root".into(),
            password: "root".into(),
            server_port: 22,
            use_password: true,
            private_key_path: "".into(),
            use_passphrase: false,
            passphrase: "".into(),
        };
        
        let validated = server.validate().unwrap();

        let (tx, rx) = std::sync::mpsc::channel::<ExecutionState>();

        validated
            .set_execution_channel(tx)
            .save();

        for msg in rx {
            match msg {
                ExecutionState::Message(text) => println!("{}", text),
                ExecutionState::Done => {
                    println!("✅ Test completed successfully");
                    break;
                }
                ExecutionState::Error(e) => {
                    panic!("❌ Error: {}", e);
                }
            }
        }

        setup::clean_config();

    }

    #[test]
    fn test_get_servers_file_not_exists()
    {
        init_config(Some("test_get_servers_file_not_exists".to_string()));
        let path= config_path(
            "servers.toml".to_string()
        );
        let servers= Server::get_servers();
        assert_eq!(path.exists(), true);
        assert_eq!(servers.is_empty(), true);
        setup::clean_config();
    }

    #[test]
    fn test_get_servers_file_exists()
    {
        init_config(Some("test_get_servers_file_exists".to_string()));

        let toml = r#"# WorldServers - Registered servers
[[servers]]
server_name = "Servidor Web"
server_ip = "10.0.0.5"
ssh_user = "admin"
private_key_path = "/home/user/.ssh/id_rsa"
use_passphrase = false
passphrase = ""
server_port = 22
use_password = false
password = ""
"#;

        std::fs::write(
            setup::config_path("servers.toml".to_string()), toml
        ).expect("Failed to create servers.toml");

        let servers= Server::get_servers();

        assert_eq!(servers.len(), 1);
        assert_eq!(servers.first().unwrap().server_name, "Servidor Web");

        setup::clean_config();
    }

}
