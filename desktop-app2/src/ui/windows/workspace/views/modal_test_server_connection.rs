use eframe::egui;
use crate::ui::{utilities::ProcessState, windows::workspace::workspace_window::WorkspaceWindow};

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    egui::Area::new(egui::Id::new("modal_server_test_connection_view"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui.ctx(), |ui| {
            let bg = ui.ctx().viewport_rect();
            ui.painter().rect_filled(bg, 0.0, egui::Color32::from_black_alpha(100));
        });


    let width = 560.0;
    let height= 200.0;

    egui::Window::new("Test Connection Server")
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(false)  // Oculta la X y la barra
        .fixed_size(egui::vec2(width, height))
        .max_height(height)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(egui::vec2(width, height)); 

            if window.server_test_connection.process_state == ProcessState::Running{
                ui.ctx().request_repaint();
            }
            ui.set_width(ui.available_width());
            ui.set_height(ui.available_height());
            egui::Frame::NONE
                .fill(egui::Color32::from_rgb(0, 0, 0))
                .corner_radius(6)
                .inner_margin(egui::Margin::symmetric(8, 6))
                .show(ui, |ui| {

                    if window.server_test_connection.process_state == ProcessState::Running {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::Spinner::new()
                                    .size(12.0)
                            );
                            ui.label(
                                egui::RichText::new("Executing")
                                    .size(12.0)
                                    .monospace()
                            );
                        });
                        ui.add_space(10.0);
                    }else {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("Log: ")
                                    .size(12.0)
                                    .monospace()
                            );
                        });
                        ui.add_space(10.0);
                    }

                    window.style.log_panel(
                        ui, &window.server_test_connection.process_log, 
                        window.server_test_connection.process_state == ProcessState::Running
                    );

                    if ProcessState::Running != window.server_test_connection.process_state {
                        ui.separator();
                    }
                    if let ProcessState::ProcessError(error) = &window.server_test_connection.process_state {
                        window.style.error_panel(ui, error);
                    }


                    if window.server_test_connection.process_state == ProcessState::Done {
                        if window.style.button(ui, "Accept", 1.0).clicked(){
                            window.test_server_accept();

                        }
                    }
                });
        });

}