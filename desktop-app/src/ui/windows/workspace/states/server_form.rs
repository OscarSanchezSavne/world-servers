use std::sync::{mpsc::Receiver};

use uuid::Uuid;

use crate::{core::server::manager, ui::utilities::{ExecutionState, ProcessState}};

pub struct ServerForm{
    pub show: bool,
    pub create: bool,
    pub uuid: Option<Uuid>,
    pub server_name: String,
    pub server_ip: String,
    pub server_port: String,
    pub use_password: bool,
    pub password: String,
    pub ssh_user: String,
    pub private_key_path: String,
    pub use_passphrase: bool,
    pub passphrase: String,
    pub process_log: Vec<String>,
    pub process_state: ProcessState,
    pub execution_receiver: Option<Receiver<ExecutionState>>,
}


impl ServerForm {
    pub fn new() -> Self {
        Self {
            uuid: None,
            show: false,
            create: true,
            server_name: "".to_string(),
            server_ip: "".to_string(),
            server_port: "22".to_string(),
            ssh_user: "".to_string(),
            use_password: false,
            password: "".to_string(),
            private_key_path: "".to_string(),
            use_passphrase: false,
            passphrase: "".to_string(),
            
            process_log: Vec::new(),
            process_state: ProcessState::Idle,
            execution_receiver: None
        }
    }
    
    pub fn to_server(&self)-> manager::Server
    {
        manager::Server{
            uuid: self.uuid,
            server_name: self.server_name.clone(),
            server_ip: self.server_ip.clone(),
            server_port: self.server_port.parse::<u16>().unwrap(),
            ssh_user: self.ssh_user.clone(),
            use_password: self.use_password,
            password: self.password.clone(),
            private_key_path: self.private_key_path.clone(),
            use_passphrase: self.use_passphrase,
            passphrase: self.passphrase.clone(),
        }
    }
    
    pub fn from_server(server: manager::Server)->  Self
    {
        Self { 
            uuid: server.uuid, 
            show: false, 
            create: false, 
            server_name: server.server_name,
            server_ip: server.server_ip,
            server_port: server.server_port.to_string(),
            use_password: server.use_password,
            password: server.password,
            ssh_user: server.ssh_user,
            private_key_path: server.private_key_path,
            use_passphrase: server.use_passphrase,
            passphrase: server.passphrase,
            process_log: Vec::new(),
            process_state: ProcessState::Idle,
            execution_receiver: None,
        }
    }

    pub fn validate_state_execution(&mut self)
    {
        if self.process_state == ProcessState::Running {
            if let Some(ref rx) = self.execution_receiver {
                while let Ok(msg) = rx.try_recv() {
                    match msg {
                        ExecutionState::Message(text) => self.process_log.push(text),
                        ExecutionState::Done => {
                            self.process_state= ProcessState::Done;
                            break;
                        }
                        ExecutionState::Error(e) => {
                            self.process_state= ProcessState::ProcessError(e);
                            break;
                        }
                    }
                }
            }

            if self.process_state != ProcessState::Running  {
                self.execution_receiver= None;
            }

        }


    }

}