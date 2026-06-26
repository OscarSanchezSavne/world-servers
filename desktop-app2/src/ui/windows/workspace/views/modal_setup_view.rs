use eframe::egui;

use crate::ui::windows::workspace::workspace_window::WorkspaceWindow;

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    let mut open = true;
    egui::Area::new(egui::Id::new("modal_setup"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui.ctx(), |ui| {
            let bg = ui.ctx().viewport_rect();
            ui.painter().rect_filled(bg, 0.0, egui::Color32::from_black_alpha(100));
        });

    egui::Window::new("Setup")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .title_bar(window.setup.configured)  // Oculta la X y la barra
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ui.ctx(), |ui| {
            ui.set_min_size(egui::vec2(560.0, 220.0)); 

            let mut host_str = window.setup_state.central_host.clone();
            if window.style.text_input(ui, "Host / IP", &mut host_str).changed() {
                window.setup_state.central_host = host_str;
            }
            window.style.line_space(ui);

            let mut port_str = window.setup_state.central_port.to_string();
            if window.style.text_input(ui, "Port", &mut port_str).changed() {
                if let Ok(port) = port_str.parse::<u16>() {
                    window.setup_state.central_port = port;
                }
            }

            window.style.line_space(ui);
            window.style.line_space(ui);
            window.style.line_space(ui);
            window.style.line_space(ui);


            ui.horizontal(|ui| {
                if window.style.button(ui, "Save", 2.0).clicked(){
                    window.save_setup();
                }

                if window.setup.configured{
                    if window.style.button(ui, "Cancel", 1.0).clicked(){
                        window.cancel_setup();
                    }
                }
            });

        });
        if !open {
            window.setup_state.show_setup = false;
        }

}