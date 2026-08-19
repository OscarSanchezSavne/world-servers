mod ui;
mod visualizer;
mod core;

use crate::{ui::windows::workspace::workspace_window::{self}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let exe_dir = std::env::current_exe()?
        .parent()
        .unwrap()
        .to_path_buf();

    let asset_root = if exe_dir.join("assets").exists() {
        exe_dir
    } else if let Ok(appdir) = std::env::var("APPDIR") {
        std::path::PathBuf::from(appdir).join("usr/share/worldservers")
    } else {
        exe_dir.join("../../AppDir/usr/share/worldservers")
    };

    unsafe{
        std::env::set_var("BEVY_ASSET_ROOT", asset_root.to_string_lossy().to_string());
    }

    core::system::setup::init_config(None);
    if args.len() > 1 && args[1] == "--visualizer" {
        visualizer::my_world_lifecycle::big_bang();
        Ok(())
    } else {
        workspace_window::run();
        Ok(())
    }
    
}

