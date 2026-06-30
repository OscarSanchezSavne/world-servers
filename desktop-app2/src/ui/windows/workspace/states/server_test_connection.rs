use std::sync::{mpsc::Receiver};

use crate::{ui::utilities::{ExecutionState, ProcessState}};

pub struct ServerTestConnection{
    pub show: bool,
    pub process_log: Vec<String>,
    pub process_state: ProcessState,
    pub execution_receiver: Option<Receiver<ExecutionState>>,
}


impl ServerTestConnection {
    pub fn new() -> Self {
        Self {
            show: false,
            process_log: Vec::new(),
            process_state: ProcessState::Idle,
            execution_receiver: None
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