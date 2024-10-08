use bevy::prelude::*;
use std::sync::{Mutex, Arc};
use tokio::sync::mpsc::Sender;
use crate::requests::ClientPacket;
use crate::components::network_writer::NetworkWriter;
use crate::requests::skill_execute::SkillExecuteRequest;
use crate::requests::skill_prepare::SkillPrepareRequest;

pub struct TcpServerPlugin;

impl Plugin for TcpServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(First, process_socket_queue);
        app.add_systems(Last, clear_skill_execute);
        app.add_systems(Last, clear_skill_prepare);
    }
}

#[derive(Component)]
pub struct UserAddr {
    pub socket_addr: String
}

#[derive(Resource, Debug)]
pub struct SocketQueue {
    pub queue: Arc<Mutex<Vec<SocketPair>>>
}

#[derive(Debug)]
pub struct SocketPair(pub String, pub SocketMessage);

#[derive(Debug)]
pub enum SocketMessage {
    Connected(Sender<Vec<u8>>),
    Packet(ClientPacket),
    Disconnected,
}

fn process_socket_queue(mut commands: Commands, queue: ResMut<SocketQueue>, entities: Query<(Entity, &UserAddr)>) {
    let mut queue = queue.queue.lock().unwrap();
    queue.drain(0..).for_each(|connection_pair| {
        match connection_pair.1 {
            SocketMessage::Connected(socket_writer) => {
                commands.spawn((NetworkWriter { socket_writer }, UserAddr { socket_addr: connection_pair.0.clone() }));
            },
            SocketMessage::Packet(client_packet) => {
                for (entity, user_addr) in &entities {
                    if user_addr.socket_addr == connection_pair.0 {
                        match client_packet {
                            ClientPacket::ServerSelect(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::Authenticate(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::DeleteCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::RestoreDeletedCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::CreateCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::SelectCharacter(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::ChatMessage(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerWalk(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::PlayerStopWalking(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::Emote(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::EquipItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::UnequipItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::UseItem(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::NormalHit(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::SkillPrepare(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            ClientPacket::SkillExecute(ref client_packet) => {commands.entity(entity).insert(client_packet.clone());},
                            _ => {
                                if let ClientPacket::Unknown(ref client_packet) = client_packet {
                                    if *client_packet.buffer.get(2).unwrap() != 253 {
                                        println!("{:?}", client_packet)
                                    }
                                } else {
                                    println!("{:?}", client_packet)
                                }
                            },
                        };
                    }
                }
            },
            SocketMessage::Disconnected => (),
        };
    })
}

fn clear_skill_prepare(mut commands: Commands, query: Query<Entity, With<SkillPrepareRequest>>) {
    for entity in &query {
        commands.entity(entity).remove::<SkillPrepareRequest>();
    }
}

fn clear_skill_execute(mut commands: Commands, query: Query<Entity, With<SkillExecuteRequest>>) {
    for entity in &query {
        commands.entity(entity).remove::<SkillExecuteRequest>();
    }
}