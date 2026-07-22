mod ui;
mod visualizer;
mod core;

use crate::{ui::windows::workspace::workspace_window::{self}};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--visualizer" {
        visualizer::run::world_3d();
        Ok(())
    } else {
        workspace_window::run();
        Ok(())
    }
    
}

