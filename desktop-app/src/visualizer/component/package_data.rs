use bevy::prelude::*;
use uuid::Uuid;
use crate::{ui::utilities::TcpdumpPacket};


#[derive(Component, Debug, Clone)]
pub struct PackageData{
    pub target: Vec3,
}

#[derive(Clone, Debug)]
pub struct RawPackage{
    pub server_uuid: Uuid,
    pub package_data: TcpdumpPacket
}