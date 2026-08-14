use std::sync::{Mutex, mpsc};

use bevy::prelude::*;

use crate::visualizer;

#[derive(Resource)]
pub struct PackageDataQueue {
    packages: Vec<visualizer::component::package_data::RawPackage>,
    pub tx: mpsc::Sender<visualizer::component::package_data::RawPackage>,
    pub rx: Mutex<mpsc::Receiver<visualizer::component::package_data::RawPackage>>,
}

impl PackageDataQueue {
    pub fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<visualizer::component::package_data::RawPackage>();

        Self{
            packages: Vec::new(),
            tx, 
            rx: Mutex::new(rx),
        }
    }

    pub fn add(&mut self, raw_package: visualizer::component::package_data::RawPackage)
    {
        let total_packages = self.packages.iter().filter(|package| {
            package.server_uuid == raw_package.server_uuid
                && package.package_data.inbound == raw_package.package_data.inbound
                && package.package_data.internal == raw_package.package_data.internal
        });
        if total_packages.count() < 30{
            self.packages.push(raw_package);
        }
    }

    pub fn take(&mut self)-> Vec<visualizer::component::package_data::RawPackage>
    {
        let packages= std::mem::take(&mut self.packages);
        packages
    }

}
