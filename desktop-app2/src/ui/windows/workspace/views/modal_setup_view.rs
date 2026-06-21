use eframe::egui;

use crate::ui::windows::workspace::workspace_window::WorkspaceWindow;

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    let mut open = true;
    egui::Area::new(egui::Id::new("modal_overlay"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui.ctx(), |ui| {
            let bg = ui.ctx().viewport_rect();
            ui.painter().rect_filled(bg, 0.0, egui::Color32::from_black_alpha(100));
        });

    egui::Window::new("Network Setup")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
                ui.set_min_size(egui::vec2(560.0, 500.0)); 

            window.style.line_space(ui);
            window.style.title(ui, "Network Configuration");

            let mut host_str = window.setup_state.central_host.clone();
            if window.style.text_input(ui, "Host / IP", &mut host_str) {
                window.setup_state.central_host = host_str;
            }
            window.style.line_space(ui);

            let mut port_str = window.setup_state.central_port.to_string();
            if window.style.text_input(ui, "Port", &mut port_str) {
                if let Ok(port) = port_str.parse::<u16>() {
                    window.setup_state.central_port = port;
                }
            }

            window.style.line_space(ui);
            window.style.line_space(ui);

            window.style.button(ui, "Save");

        });
        if !open {
            window.setup_state.show_setup = false;
        }

}