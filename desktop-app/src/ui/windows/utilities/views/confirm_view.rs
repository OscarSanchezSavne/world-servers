use eframe::egui;
use crate::ui::{utilities::ProcessState, windows::workspace::workspace_window::WorkspaceWindow};

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    egui::Area::new(egui::Id::new("confirm"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui.ctx(), |ui| {
            let bg = ui.ctx().viewport_rect();
            ui.painter().rect_filled(bg, 0.0, egui::Color32::from_black_alpha(100));
        });


    let width = 560.0;
    let height= 100.0;

    egui::Window::new("Confirm")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(true)
        .fixed_size(egui::vec2(width, height))
        .max_height(height)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(egui::vec2(width, height)); 

            if window.server_test_connection.process_state == ProcessState::Running{
                ui.ctx().request_repaint();
            }
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(window.confirm.label.to_string())
                        .size(12.0)
                        .monospace()
                );
            });
            ui.add_space(30.0);

            ui.horizontal(|ui: &mut egui::Ui| {
                if window.style.button(ui, "Accept", 2.0).clicked(){
                    window.confirm.accept();

                }
                if window.style.button(ui, "Cancel", 1.0).clicked(){
                    window.confirm.cancel();

                }
            });
        });

}