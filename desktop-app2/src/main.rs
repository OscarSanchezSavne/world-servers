mod ui;
mod core;
use eframe::egui;

use crate::{core::system, ui::windows::workspace::workspace_window::WorkspaceWindow};


fn main() -> eframe::Result {
    let icon = ui::utilities::load_favicon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 650.0])
            .with_icon(icon),
        ..Default::default()
    };

    system::setup::init_config(None);

    eframe::run_native(
        "WorldServers",
        options,
        Box::new(WorkspaceWindow::create)
    )
}

