use eframe::egui::{self, TextureHandle};

use crate::core::system;
use crate::ui::style_global;
use crate::ui::style_global::StyleGlobal;
use crate::ui::utilities::*;
use crate::ui::windows::workspace::states::setup_state;
use crate::ui::windows::workspace::views::workspace_view;


pub struct WorkspaceWindow{
    pub logotipo: TextureHandle,
    pub isotipo: TextureHandle,
    pub style: StyleGlobal,
    pub setup_state: setup_state::SetupState,
}

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
        let logotipo =load_texture(ctx, style_global::LOGOTIPO_BYTES);
        let isotipo =load_texture(ctx, style_global::ISOTIPO_BYTES);
        let style= StyleGlobal::new();
        let setup= system::setup::load_config();
        let setup_state= setup_state::SetupState::new(setup);
        Self{
            logotipo,
            isotipo,
            setup_state,
            style
        }
    }

}


impl eframe::App for WorkspaceWindow {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        workspace_view::render(ui, self)
    }
}
