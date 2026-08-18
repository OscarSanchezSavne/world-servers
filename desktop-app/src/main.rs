mod ui;
mod visualizer;
mod core;

use crate::{ui::windows::workspace::workspace_window::{self}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    let base_dir = if let Ok(appdir) = std::env::var("APPDIR") {
        appdir
    } else {
        let exe = std::env::current_exe()?;
        exe.parent().unwrap()
            .join("../../AppDir")
            .to_string_lossy().to_string()
    };

    unsafe{
        std::env::set_var("BEVY_ASSET_ROOT", format!("{}/usr/share/worldservers", base_dir));
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

