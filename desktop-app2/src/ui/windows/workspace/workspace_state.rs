pub struct WorkspaceState {
    pub show_setup: bool,
    pub central_host: String,
    pub central_port: u16,
}

impl WorkspaceState {
    pub fn new() -> Self {
        Self {
            show_setup: false, 
            central_host: String::new(),
            central_port: 0,
        }
    }
}