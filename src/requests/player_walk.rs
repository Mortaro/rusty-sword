use crate::framework::packet::Packet;
use bevy::prelude::*;

pub const HEADER: u8 = 94;

#[derive(Component, Debug, Clone)]
pub struct PlayerWalkRequest {
    pub delta_x: u8,
    pub delta_y: u8,
    pub delta_z: u8,
}

impl From<&mut Packet> for PlayerWalkRequest {
    fn from(packet: &mut Packet) -> Self {
        let delta_x = packet.get_u8();
        let delta_y = packet.get_u8();
        let delta_z = packet.get_u8();
        PlayerWalkRequest { delta_x, delta_y, delta_z }
    }
}