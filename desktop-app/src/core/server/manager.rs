use std::{io::Read, net::TcpStream, sync::mpsc::{Sender}, thread, time::Duration};

use serde::{Deserialize, Serialize};
use ssh2::{ExtendedData, Session};
use uuid::Uuid;

use crate::{core::system::{self, crypto, setup}, ui::utilities::{self, ExecutionState, ServerMetrics, ServerTraffic}};


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
        if let Some(_) = self.uuid{
            if servers.iter().any(
                |s| s.server_name == self.server_name && s.uuid != self.uuid
            ) {
                return Err("A server with this name already exists".into());
            }
            if servers.iter().any(
                |s| s.server_ip == self.server_ip && s.uuid != self.uuid
            ) {
                return Err("A server with this IP address already exists".into());
            }
        }else{
            if servers.iter().any(|s| s.server_name == self.server_name) {
                return Err("A server with this name already exists".into());
            }
            if servers.iter().any(|s| s.server_ip == self.server_ip) {
                return Err("A server with this IP address already exists".into());
            }
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
        let server_uuid= self.uuid.unwrap().clone();
        tx.send(ExecutionState::Message(server_uuid, "Start connection".to_string())).unwrap();

        tx.send(ExecutionState::Message(server_uuid, "Connected to server".to_string())).unwrap();
        let session= self.open_session(Some(&tx));
        let mut channel = session.channel_session().unwrap();
        let command = "command -v tcpdump"; 
        channel.exec(&command).unwrap();

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        channel.wait_close().unwrap();

        let exit_code = channel.exit_status().unwrap();

        if exit_code == 0 && !output.trim().is_empty() {
            tx.send(ExecutionState::Message(server_uuid, format!("Tcpdump installed at: {}", output.trim()))).unwrap();
        } else {
            panic!("Tcpdump is NOT installed (sudo apt install tcpdump)");
        }
        
        channel.wait_close().unwrap();
        tx.send(ExecutionState::Message(server_uuid, format!("Connection to server successful"))).unwrap();

    }


    pub fn async_test_ssh_connection(self, sender: Sender<ExecutionState>){
        let prefix = setup::get_config_prefix();
        let server_uuid= self.uuid.unwrap().clone();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);

            let sender_result= sender.clone();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.test_ssh_connection(sender);
            }));

            let outcome = match result {
                Ok(_) => {
                    ExecutionState::Done(server_uuid)
                },
                Err(e) => {
                    ExecutionState::Error(server_uuid, utilities::parse_error(e))
                }
            };
            if let Err(e) = sender_result.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }

    pub fn delete(&self) 
    {
        let mut servers = Server::get_servers();
        servers.retain(|s| s.uuid != self.uuid);

        let content = format!(
            "# WorldServers - Registered servers\n{}",
            toml::to_string(&ServersList { servers }).expect("Failed to serialize servers")
        );

        let encrypted = crypto::obfuscate(&content);
        
        std::fs::write(
            setup::config_path("servers.toml".to_string()), 
            encrypted
        )
        .expect("Failed to write servers.toml");
    }

    pub fn run_tcpdump(&self, tx: &Sender<ServerTraffic>, inbound: bool) {
        let session = self.open_session(None);
        let server_uuid = self.uuid.unwrap().clone();

        let command = if inbound {
            r#"sudo -n tcpdump -i $(ip -o -4 route show to default | awk '{print $5}') -nn -tt -l inbound and not port 22"#
        } else {
            r#"sudo -n tcpdump -i $(ip -o -4 route show to default | awk '{print $5}') -nn -tt -l outbound and not port 22"#
        };

        let mut channel = match session.channel_session() {
            Ok(channel) => channel,

            Err(e) => {
                let _ = tx.send(ServerTraffic::Error(server_uuid, format!("Unable to open SSH channel: {}", e)));
                return;
            }
        };

        if let Err(e) = channel.handle_extended_data(ExtendedData::Merge) {
            let _ = tx.send(ServerTraffic::Error(server_uuid, format!("Unable to merge stderr: {}", e)));
            return;
        }

        if let Err(e) = channel.exec(command) {
            let _ = tx.send(ServerTraffic::Error(server_uuid, format!("Unable to start tcpdump: {}", e)));
            return;
        }

        println!("{} tcpdump started, streaming packets...", server_uuid);

        let mut buf = [0u8; 4096];
        let mut leftover = String::new();
        loop {
            match channel.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    leftover.push_str(&String::from_utf8_lossy(&buf[..n]));
                    while let Some(pos) = leftover.find('\n') {
                        let line = leftover[..pos].trim().to_string();
                        leftover = leftover[pos + 1..].to_string();

                        if line.is_empty() {
                            continue;
                        }

                        let parts: Vec<&str> = line.split_whitespace().collect();
                        if parts.len() < 6 || parts[1] != "IP" {
                            continue;
                        }

                        let dst_ip = tcpdump_ip(parts[4]);
                        let src_ip = tcpdump_ip(parts[2]);

                        let size = parts
                            .windows(2)
                            .find(|w| w[0] == "length")
                            .and_then(|w| w[1].parse::<u64>().ok())
                            .unwrap_or(0);

                        if size == 0 {
                            continue; // saltas ACKs, FINs, SYNs, etc.
                        }
                        tx.send(ServerTraffic::Package(
                            server_uuid,
                            utilities::TcpdumpPacket {src_ip, dst_ip, size, inbound, internal: None},
                        )).unwrap();
                    }
                }
                Err(e) => {
                    tx.send(ServerTraffic::Error(server_uuid, format!("tcpdump read error: {}", e))).unwrap();
                    break;
                }
            }
        }



        channel.wait_close().unwrap();

        println!("{} tcpdump finished", server_uuid);
    }


    pub fn async_run_tcpdump(self, sender: Sender<ServerTraffic>, inbound: bool) {
        let prefix = setup::get_config_prefix();
        let server_uuid = self.uuid.unwrap().clone();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);

            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.run_tcpdump(&sender, inbound);
            }));

            match result {
                Ok(_) => return,
                Err(e) => ExecutionState::Error(server_uuid, utilities::parse_error(e)),
            };

        });
    }

    fn open_session(&self, tx: Option<&Sender<ExecutionState>>) -> Session {
        let server_uuid = self.uuid.unwrap().clone();
        let send_channel = |msg: &str| {
            if let Some(channel) = tx {
                channel.send(ExecutionState::Message(server_uuid, msg.to_string())).unwrap();
            }
        };

        let tcp = TcpStream::connect_timeout(
            &format!("{}:{}", self.server_ip, self.server_port).parse().unwrap(),
            Duration::from_secs(10)
        ).expect("Connection timed out. Server did not  respond");

        let mut session = Session::new().expect("Failed to create SSH session");
        
        send_channel("SSH session created");

        session.set_tcp_stream(tcp);
        session.handshake().expect("SSH handshake failed. Check the server address");
        send_channel("SSH handshake completed");

        if self.use_password {
            session.userauth_password(
                &self.ssh_user,
                &self.password,
            ).expect("Password authentication failed. Check your credentials");
            send_channel("Password authentication successful");
        }else{
            let passphrase = if self.use_passphrase{
                Some(self.passphrase.as_str())
            }else { None };

            session.userauth_pubkey_file(
                &self.ssh_user, None, 
                &expand_path(&self.private_key_path), passphrase,
            ).expect("SSH authentication failed. Check your credentials");
            send_channel("SSH authentication successful");
        }

        if !session.authenticated() {
            panic!("Authentication rejected by server");
        }
        session
    }


    pub fn get_metrics_avg(&self) -> (f32, f32, f32) {
        let session = self.open_session(None);

        // Una sola ejecución: CPU, RAM, Discos
        let command = r#"
        LC_ALL=C top -bn2 -d 1 | awk '/%Cpu/ {cpu=100-$8} END {print cpu}';
        free | awk '/Mem:/ {print $3/$2}';
        df -P -T | awk '
        NR > 1 &&
        $1 ~ "^/dev/" &&
        $2 != "squashfs" {
            total += $3;
            used += $4;
        }
        END {
            if (total > 0)
                print used / total;
            else
                print 0;
        }'
        "#;

        let mut channel = session.channel_session().unwrap();
        channel.exec(command).unwrap();

        let mut output = String::new();
        channel.read_to_string(&mut output).unwrap();
        channel.wait_close().unwrap();

        let values: Vec<f32> = output
            .lines()
            .filter_map(|line| line.trim().parse::<f32>().ok())
            .collect();

        if values.len() < 3 {
            return (0.0, 0.0, 0.0);
        }

        let cpu = values[0] / 100.0;
        let ram = values[1];
        let disk = values[2];

        (
            ((cpu * 100.0).round() / 100.0),
            ((ram * 100.0).round() / 100.0),
            ((disk * 100.0).round() / 100.0),
        )
    }


    pub fn async_get_metrics_avg(self, sender: Sender<ServerMetrics>){
        let prefix = setup::get_config_prefix();
        let server_uuid= self.uuid.unwrap().clone();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);

            let sender_result= sender.clone();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.get_metrics_avg()
            }));

            let outcome = match result {
                Ok((cpu, ram, disk)) => {
                    ServerMetrics::Done(server_uuid, cpu, ram, disk)
                },
                Err(e) => {
                    ServerMetrics::Error(server_uuid, utilities::parse_error(e))
                }
            };
            if let Err(e) = sender_result.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }

}


fn tcpdump_ip(value: &str) -> String {
    let value = value.trim_end_matches(':');

    if value.matches('.').count() <= 3 {
        return value.to_string();
    }

    match value.rfind('.') {
        Some(pos) => value[..pos].to_string(),
        None => value.to_string(),
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
        
        let server_uuid= self.server.uuid.unwrap().clone();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.server.test_ssh_connection(tx.clone());
                self.write_to_file(tx.clone());
            }));

            let outcome = match result {
                Ok(_) => {
                    ExecutionState::Done(server_uuid)
                },
                Err(e) => {
                    ExecutionState::Error(server_uuid, utilities::parse_error(e))
                }
            };
            if let Err(e) = tx.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }


    fn write_to_file(&self, tx: Sender<ExecutionState>) 
    {
        let server_uuid= self.server.uuid.unwrap().clone();
        tx.send(ExecutionState::Message(server_uuid, "Saving server to file...".to_string())).unwrap();
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
        tx.send(ExecutionState::Message(server_uuid, "Server saved successfully".to_string())).unwrap();
    }

    pub fn update(mut self) {
        let tx = self.execution_channel
            .expect("[SystemError] No execution channel set");

        self.execution_channel= None;
        let prefix = setup::get_config_prefix();
        let server_uuid= self.server.uuid.unwrap().clone();
        thread::spawn(move || {
            setup::set_config_prefix(prefix);
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                self.server.test_ssh_connection(tx.clone());
                self.add_to_file(tx.clone());
            }));

            let outcome = match result {
                Ok(_) => {
                    ExecutionState::Done(server_uuid)
                },
                Err(e) => {
                    ExecutionState::Error(server_uuid, utilities::parse_error(e))
                }
            };
            if let Err(e) = tx.send(outcome) {
                eprintln!("[Warning] Failed to send execution result: {}", e);
            }
        });
    }


    fn add_to_file(&self, tx: Sender<ExecutionState>) 
    {
        let server_uuid= self.server.uuid.unwrap().clone();
        tx.send(ExecutionState::Message(server_uuid, "Saving server to file...".to_string())).unwrap();
        let mut servers = Server::get_servers();
        if let Some(pos) = servers.iter().position(|s| s.uuid == self.server.uuid) {
            servers[pos] = self.server.clone();
            tx.send(ExecutionState::Message(server_uuid, "Server updated successfully".to_string())).unwrap();
        }
        
        let content = format!(
            "# WorldServers - Registered servers\n{}",
            toml::to_string(&ServersList { servers }).expect("Failed to serialize servers")
        );

        let encrypted = crypto::obfuscate(&content);
        
        std::fs::write(
            setup::config_path("servers.toml".to_string()), 
            encrypted
        )
        .expect("Failed to write servers.toml");
        tx.send(ExecutionState::Message(server_uuid, "Server saved successfully".to_string())).unwrap();
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