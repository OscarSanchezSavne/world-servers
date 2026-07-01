use eframe::egui;
use crate::ui::{utilities::ProcessState, windows::workspace::workspace_window::WorkspaceWindow};

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    let mut open = true;
    egui::Area::new(egui::Id::new("modal_server_form_view"))
        .fixed_pos(egui::Pos2::ZERO)
        .show(ui.ctx(), |ui| {
            let bg = ui.ctx().viewport_rect();
            ui.painter().rect_filled(bg, 0.0, egui::Color32::from_black_alpha(100));
        });


    let width = 560.0;
    let mut height= 200.0;
    if window.server_form.process_state == ProcessState::Idle {
        height = 500.0;
    }

    egui::Window::new("New Server")
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .title_bar(window.server_form.process_state != ProcessState::Running)  // Oculta la X y la barra
        .fixed_size(egui::vec2(width, height))
        .max_height(height)
        .show(ui.ctx(), |ui| {
            ui.set_min_size(egui::vec2(width, height)); 
            if matches!(window.server_form.process_state, ProcessState::Idle | ProcessState::ValidateError(_)) {
                
                let mut server_name = window.server_form.server_name.clone();

                window.style.info_panel(
                    ui, "Register a new server",
                    "Fill in the details below to connect to your new server via SSH."
                );
                window.style.line_space(ui);

                if window.style.text_input_with_hint(
                    ui, "Give your server a unique name", &mut server_name, "e.g., Production Server"
                ).changed() {
                    window.server_form.server_name = server_name;
                }
                window.style.line_space(ui);

                let mut server_ip = window.server_form.server_ip.clone();
                if window.style.text_input_with_hint(
                    ui, "Enter server IP address", &mut server_ip, "e.g., 192.168.1.100"
                ).changed() {
                    window.server_form.server_ip = server_ip;
                }
                window.style.line_space(ui);

                let mut server_port = window.server_form.server_port.clone();
                if window.style.text_input_with_hint(
                    ui, "Enter server Port", &mut server_port, ""
                ).changed() {
                    if server_port.is_empty() || server_port.parse::<u16>().is_ok() {
                        window.server_form.server_port = server_port;
                    }
                }
                window.style.line_space(ui);

                let mut ssh_user = window.server_form.ssh_user.clone();
                if window.style.text_input_with_hint(
                    ui, "Username for connection", &mut ssh_user, "e.g., root or admin"
                ).changed() {
                    window.server_form.ssh_user = ssh_user;
                }
                window.style.line_space(ui);


                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.selectable_value(&mut window.server_form.use_password, false, "Login with SSH key");
                        ui.selectable_value(&mut window.server_form.use_password, true, "Login with Password");
                    });
                });
                window.style.line_space(ui);

                if window.server_form.use_password {
                    let mut password = window.server_form.password.clone();
                    if window.style.text_input_with_hint(
                        ui, "Password for connection", &mut password, "Enter your password"
                    ).changed() {
                        window.server_form.password = password;
                    }
                    window.style.line_space(ui);

                } else {
                    let mut private_key_path = window.server_form.private_key_path.clone();
                    if window.style.text_input_with_hint(
                        ui, "Path to your private SSH key", &mut private_key_path, "e.g., ~/.ssh/id_rsa"
                    ).changed() {
                        window.server_form.private_key_path = private_key_path;
                    }
                    window.style.line_space(ui);


                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.selectable_value(&mut window.server_form.use_passphrase, true, "SSH key with PassPhrase");
                            ui.selectable_value(&mut window.server_form.use_passphrase, false, "SSH key without PassPhrase");
                        });
                    });
                    window.style.line_space(ui);

                    if window.server_form.use_passphrase{
                        let mut passphrase = window.server_form.passphrase.clone();
                        if window.style.text_input_with_hint(
                            ui, "Passphrase for your SSH key", &mut passphrase, "Enter your SSH key passphrase"
                        ).changed() {
                            window.server_form.passphrase = passphrase;
                        }
                        window.style.line_space(ui);
                    }

                    if let ProcessState::ValidateError(msg) = &window.server_form.process_state {
                        window.style.error_panel(ui, msg);
                    }
                }

                ui.separator();
                window.style.line_space(ui);
                ui.horizontal(|ui: &mut egui::Ui| {
                    if window.style.button(ui, "Cancel", 2.0).clicked(){
                        window.cancel_server_form();
                    }
                    if window.style.button(ui, "Save", 1.0).clicked(){
                        window.save_server_form();
                    }
                });

            } 
            
            if matches!(
                &window.server_form.process_state, ProcessState::Running | ProcessState::ProcessError(_) | ProcessState::Done
            ) {
                if window.server_form.process_state == ProcessState::Running{
                    ui.ctx().request_repaint();
                }
                ui.set_width(ui.available_width());
                ui.set_height(ui.available_height());
                egui::Frame::NONE
                    .fill(egui::Color32::from_rgb(0, 0, 0))
                    .corner_radius(6)
                    .inner_margin(egui::Margin::symmetric(8, 6))
                    .show(ui, |ui| {

                        if window.server_form.process_state == ProcessState::Running {
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
                            ui, &window.server_form.process_log, 
                            window.server_form.process_state == ProcessState::Running
                        );

                        if ProcessState::Running != window.server_form.process_state {
                            ui.separator();
                        }
                        if let ProcessState::ProcessError(error) = &window.server_form.process_state {
                            window.style.error_panel(ui, error);
                        }


                        if window.server_form.process_state == ProcessState::Done {
                            if window.style.button(ui, "Accept", 1.0).clicked(){
                                window.cancel_server_form();
                            }
                        }

                        
                        if let ProcessState::ProcessError(_) = &window.server_form.process_state {
                            ui.horizontal(|ui: &mut egui::Ui| {
                                if window.style.button(ui, "Cancel", 2.0).clicked(){
                                    window.cancel_server_form();
                                }
                                if window.style.button(ui, "Back", 1.0).clicked(){
                                    window.server_form.process_state = ProcessState::Idle;
                                }
                            });
                        }
                    });
            }
        });
        if !open {
            window.server_form.show = false;
        }

}