use std::sync::{mpsc::Receiver};

use crate::{core::server::manager, ui::utilities::{ExecutionState, ProcessState}};

pub struct ServerForm{
    pub show: bool,
    pub create: bool,
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
            uuid: None,
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