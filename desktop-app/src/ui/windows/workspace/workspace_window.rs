use std::os::unix::process::CommandExt;
use std::sync::{Arc, Mutex};

use eframe::egui::{self, TextureHandle};
use crate::core::server::manager::{self, Server};
use crate::core::{server, system};
use crate::core::system::setup::{Setup, load_config};
use crate::ui::style_global;
use crate::ui::style_global::StyleGlobal;
use crate::ui::utilities::*;
use crate::ui::windows::utilities::states::confirm_state::ConfirmState;
use crate::ui::windows::workspace::states::server_form::ServerForm;
use crate::ui::windows::workspace::states::server_test_connection::ServerTestConnection;
use crate::ui::windows::workspace::states::setup_state;
use crate::ui::windows::workspace::views::workspace_view;


pub struct WorkspaceWindow{
    pub logotipo: TextureHandle,
    pub isotipo: TextureHandle,
    pub style: StyleGlobal,
    pub setup_state: setup_state::SetupState,
    pub setup: system::setup::Setup,
    pub server_form: ServerForm,
    pub servers: Vec<Server>,
    pub server_test_connection: ServerTestConnection,
    pub confirm: ConfirmState,
    action: Arc<Mutex<Vec<Action>>>,
}

enum Action {GetServers}

impl WorkspaceWindow {
    
    pub fn create(cc: &eframe::CreationContext<'_>) -> Result<Box<dyn eframe::App>, Box<dyn std::error::Error + Send + Sync>> {
        Self::configure_style(&cc.egui_ctx);
        Ok(Box::new(Self::new(&cc.egui_ctx)))
    }

    fn configure_style(ctx: &egui::Context) {
        let mut style = (*ctx.global_style()).clone();
        let app_syle= StyleGlobal::new();
        style.visuals.panel_fill = app_syle.color_bg_deep;
        ctx.set_global_style(style);
    }

    fn new(ctx: &egui::Context) -> Self {
        let setup= system::setup::load_config();
        let setup_state= setup_state::SetupState::new(&setup);
        Self{
            logotipo: load_texture(ctx, style_global::LOGOTIPO_BYTES),
            isotipo: load_texture(ctx, style_global::ISOTIPO_BYTES),
            setup_state,
            style: StyleGlobal::new(),
            setup,
            server_form: ServerForm::new(),
            servers: server::manager::Server::get_servers(),
            server_test_connection: ServerTestConnection::new(),
            confirm: ConfirmState::new(),
            action: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn save_setup(&mut self) {
        system::setup::save_config(& Setup{
            central_host: self.setup_state.central_host.clone(),
            central_port: self.setup_state.central_port,
            configured: true
        });
        self.setup_state.show_setup= false;
        self.setup= load_config();
    }

    pub fn cancel_setup(&mut self) {
        self.setup_state.show_setup= false;
        self.setup= load_config();
        self.setup_state= setup_state::SetupState::new(&self.setup);

    }

    pub fn open_server_new_form(&mut self) {
        self.server_form= ServerForm::new();
        self.server_form.show= true;
        self.server_form.create= true;
    }

    pub fn open_server_edit_form(&mut self, server: Server) {
        self.server_form= ServerForm::from_server(server);
        self.server_form.show= true;
        self.server_form.create= false;
    }

    pub fn test_server(&mut self, server: Server) {
        let (sender, receiver) = std::sync::mpsc::channel::<ExecutionState>();
        self.server_test_connection.show= true;
        self.server_test_connection.execution_receiver= Some(receiver);
        self.server_test_connection.process_state= ProcessState::Running;
        server.async_test_ssh_connection(sender);
    }

    pub fn delete_server(&mut self, server: &Server) 
    {
        let server_clone= server.clone();
        let post_action= self.action.clone();
        self.confirm.open(
            format!("Are you sure you want to delete server {}?", server.server_name.trim())
            ,move ||{
                server_clone.delete();
                post_action.lock().unwrap().push(Action::GetServers);
            }, 
            ||{}
        );
    }

    pub fn test_server_accept(&mut self) {
        self.server_test_connection= ServerTestConnection::new();
    }

    pub fn cancel_server_form(&mut self) {
        self.server_form= ServerForm::new();
        self.servers= manager::Server::get_servers();
    }

    pub fn save_server_form(&mut self){

        let new_server= self.server_form.to_server();
        
        let validated = match new_server.validate() {
            Ok(v) => v,
            Err(e) => {
                self.server_form.process_state = ProcessState::ValidateError(e);
                return;
            }
        };

        let (sender, receiver) = std::sync::mpsc::channel::<ExecutionState>();
        self.server_form.process_state= ProcessState::Running;
        self.server_form.execution_receiver= Some(receiver);
        self.server_form.process_log= Vec::new();

        validated
            .set_execution_channel(sender)
            .save();

    }

    pub fn update_server_form(&mut self){

        let new_server= self.server_form.to_server();
        
        let validated = match new_server.validate() {
            Ok(v) => v,
            Err(e) => {
                self.server_form.process_state = ProcessState::ValidateError(e);
                return;
            }
        };

        let (sender, receiver) = std::sync::mpsc::channel::<ExecutionState>();
        self.server_form.process_state= ProcessState::Running;
        self.server_form.execution_receiver= Some(receiver);
        self.server_form.process_log= Vec::new();

        validated
            .set_execution_channel(sender)
            .update();

    }

    fn process_actions(&mut self)
    {
        if let Ok(mut actions) = self.action.try_lock() {
            while let Some(item) = actions.pop() {
                match item {
                    Action::GetServers => {
                        self.servers = manager::Server::get_servers();
                    }
                }
            }
        }
    }

    pub fn launch_visualizer_and_exit(&self, ctx: &egui::Context) {
        let self_path = std::env::current_exe().unwrap();
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);

        let err = std::process::Command::new(self_path)
        .arg("--visualizer")
        .exec();
    
        panic!("exec failed: {}", err);
    }

}


impl eframe::App for WorkspaceWindow {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.server_form.validate_state_execution(); 
        self.server_test_connection.validate_state_execution(); 
        self.process_actions();
        
        workspace_view::render(ui, self)
            
    }
}
