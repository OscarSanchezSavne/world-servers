use std::{io::Read, net::TcpStream, sync::mpsc::Sender, thread, time::Duration};

use serde::{Deserialize, Serialize};
use ssh2::Session;
use uuid::Uuid;

use crate::{core::system::{self, crypto, setup}, ui::utilities::{self, ExecutionState}};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server{
    pub uuid: Option<Uuid>,
    pub server_name: String,
    pub server_ip: String,
    pub server_port: u16,
    pub ssh_user: String,
    pub use_password: bool,
    pub password: String,
    pub private_key_path: String,
    pub use_passphrase: bool,
    pub passphrase: String,
}

#[derive(Debug, Clone)]
pub struct ValidatedServer{
    pub server: Server,
    pub execution_channel: Option<std::sync::mpsc::Sender::<ExecutionState>>,
}


impl Default for Server {
    fn default() -> Self {
        Self {
            uuid: None,
            server_name: "".to_string(),
            server_ip: "".to_string(),
            server_port: 0,
            ssh_user: "".to_string(),
            use_password: false,
            password: "".to_string(),
            private_key_path: "".to_string(),
            use_passphrase: false,
            passphrase: "".to_string(),
        }
    }
}


#[derive(Serialize, Deserialize)]
struct ServersList {
    servers: Vec<Server>,
}

impl Server {

    pub fn get_servers() -> Vec<Server> {

        let path= system::setup::config_path(
            "servers.toml".to_string()
        );
        
        if !path.exists() {
            return Vec::new();
        }

        let content = std::fs::read_to_string(path).expect("Failed to read servers.toml");
        let decrypted = crypto::deobfuscate(&content);
        
        toml::from_str::<ServersList>(&decrypted)
            .map(|s| s.servers)
            .unwrap_or_default()
    }


    pub fn validate(self) -> Result<ValidatedServer, String> {
        if self.server_name.trim().is_empty() {
            return Err("Server Name is required".into());
        }
        if self.server_ip.trim().is_empty() {
            return Err("Server IP address is required".into());
        }

        if self.server_ip.parse::<std::net::IpAddr>().is_err() {
            return Err("Invalid IP address format".into());
        }
        
        let servers = Self::get_servers();
        if servers.iter().any(|s| s.server_name == self.server_name) {
            return Err("A server with this name already exists".into());
        }
        if servers.iter().any(|s| s.server_ip == self.server_ip) {
            return Err("A server with this IP address already exists".into());
        }

        if self.server_port == 0 {
            return Err("Port must be between 1 and 65535".into());
        }

        if self.ssh_user.trim().is_empty() {
            return Err("SSH username is required".into());
        }

        if self.use_password {
            if self.password.trim().is_empty() {
                return Err("Password is required".into());
            }
        }else{
            if self.private_key_path.trim().is_empty() {
                return Err("Private key path is required".into());
            }
            let expanded = expand_path(&self.private_key_path);
            if !expanded.exists() {
                return Err(format!("SSH key file not found at: {}", self.private_key_path));
            }
            if self.use_passphrase {
                if self.passphrase.trim().is_empty() {
                    return Err("Passphrase is required when enabled".into());
                }
            }
        }
        Ok(ValidatedServer{
            server: self,
            execution_channel: None
        })
    }


    fn test_ssh_connection(&self, tx: Sender<ExecutionState>){

        tx.send(ExecutionState::Message("Start connection".to_string())).unwrap();
        let tcp = TcpStream::connect_timeout(
            &format!("{}:{}", self.server_ip, self.server_port).parse().unwrap(),
            Duration::from_secs(10)
        ).expect("Connection timed out. Server did not  respond");

        tx.send(ExecutionState::Message("Connected to server".to_string())).unwrap();

        let mut session = Session::new().expect("Failed to create SSH session");
        tx.send(ExecutionState::Message("SSH session created".to_string())).unwrap();

        session.set_tcp_stream(tcp);
        session.handshake().expect("SSH handshake failed. Check the server address");
        tx.send(ExecutionState::Message("SSH handshake completed".to_string())).unwrap();

        if self.use_password {
            session.userauth_password(
                &self.ssh_user,
                &self.password,
            ).expect("Password authentication failed. Check your credentials");
            tx.send(ExecutionState::Message("Password authentication successful".to_string())).unwrap();
        }else{
            let passphrase = if self.use_passphrase{
                Some(self.passphrase.as_str())
            }else { None };

            session.userauth_pubkey_file(
                &self.ssh_user, None, 
                &expand_path(&self.private_key_path), passphrase,
            ).expect("SSH authentication failed. Check your credentials");
            tx.send(ExecutionState::Message("SSH authentication successful".to_string())).unwrap();
        }

        if !session.authenticated() {
            panic!("Authentication rejected by server");
        }

        let mut channel = session.channel_session().unwrap();
        let command= "date"; 
        channel.exec(&command).unwrap();
        tx.send(ExecutionState::Message(format!("Executing command: {}", command))).unwrap();

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        tx.send(ExecutionState::Message(format!("Command output: {}", output))).unwrap();
        
        channel.wait_close().unwrap();
        tx.send(ExecutionState::Message(format!("Connection to server successful: {}", output))).unwrap();

    }


    pub fn async_test_ssh_connection(self, sender: Sender<ExecutionState>){
        let prefix = setup::get_config_prefix();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);

            let sender_result= sender.clone();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.test_ssh_connection(sender);
            }));

            let outcome = match result {
                Ok(_) => {
                    ExecutionState::Done
                },
                Err(e) => {
                    ExecutionState::Error(utilities::parse_error(e))
                }
            };
            if let Err(e) = sender_result.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }

}

impl ValidatedServer {

    pub fn set_execution_channel(
        mut self, execution_channel: Sender<ExecutionState>
    ) -> Self{
        self.execution_channel= Some(execution_channel);
        self
    }

    pub fn save(mut self) {
        let tx = self.execution_channel
            .expect("[SystemError] No execution channel set");

        self.execution_channel= None;
        let prefix = setup::get_config_prefix();
        self.server.uuid= Some(Uuid::new_v4());
        thread::spawn(move || {
            setup::set_config_prefix(prefix);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.server.test_ssh_connection(tx.clone());
                self.write_to_file(tx.clone());
            }));

            let outcome = match result {
                Ok(_) => {
                    ExecutionState::Done
                },
                Err(e) => {
                    ExecutionState::Error(utilities::parse_error(e))
                }
            };
            if let Err(e) = tx.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }


    fn write_to_file(&self, tx: Sender<ExecutionState>) 
    {
        tx.send(ExecutionState::Message("Saving server to file...".to_string())).unwrap();
        let mut servers = Server::get_servers();
        servers.push(self.server.clone());
        
        let content = format!(
            "# WorldServers - Registered servers\n{}",
            toml::to_string(&ServersList { servers }).expect("Failed to save server to file")
        );

        let encrypted = crypto::obfuscate(&content);
        
        std::fs::write(
            setup::config_path("servers.toml".to_string()), 
            encrypted
        )
        .expect("Failed to write servers.toml");
        tx.send(ExecutionState::Message("Server saved successfully".to_string())).unwrap();
    }

    
}


pub(crate) fn expand_path(key_path: &str) -> std::path::PathBuf {
    if key_path.starts_with('~') && let Some(home) = std::env::var_os("HOME") {
        let mut path = std::path::PathBuf::from(home);
        let rest = key_path[1..].trim_start_matches('/');
        path.push(rest);
        path
    } else {
        std::path::PathBuf::from(key_path)
    }
}