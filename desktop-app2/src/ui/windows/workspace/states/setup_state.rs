use crate::core::system;

pub struct SetupState {
    pub central_host: String,
    pub show_setup: bool,
    pub central_port: u16,
}

impl SetupState {
    pub fn new(setup: system::setup::Setup) -> Self {
        Self {
            central_host: setup.central_host.clone(),
            central_port: setup.central_port.clone(),
            show_setup: !setup.configured,
        }
    }
    
}