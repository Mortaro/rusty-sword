use crate::components::appearence::Appearence;
use crate::components::job::Job;
use crate::components::player::Player;
use crate::components::position::Position;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 50;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum PlayerClass {
    Knight = 0,
    Mage = 1,
    Archer = 2,
}

#[derive(Debug)]
pub struct PlayerAppearResponse {
    pub player_id: u32,
    pub name: String,
    pub class: PlayerClass,
    pub is_current_player: bool,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub unknown1: Vec<u8>,
    pub weapon_index: u16,
    pub shield_index: u16,
    pub helmet_index: u16,
    pub chest_index: u16,
    pub shorts_index: u16,
    pub gloves_index: u16,
    pub boots_index: u16,
    pub unknown2: Vec<u8>,
    pub face: u8, 
    pub hair: u8, 
    pub unknown3: Vec<u8>,
}

impl PlayerAppearResponse {
    pub fn new(player: &Player, job: &Job, position: &Position, appearence: &Appearence, is_current_player: bool) -> Self {
        PlayerAppearResponse { 
            player_id: player.id, 
            name: appearence.name.clone(), 
            class: match job.class {
                0 => PlayerClass::Knight,
                1 => PlayerClass::Mage,
                _ => PlayerClass::Archer,
            }, 
            is_current_player,
            x: position.x, 
            y: position.y, 
            z: position.z, 
            unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
            weapon_index: appearence.weapon_index, 
            shield_index: appearence.shield_index, 
            helmet_index: appearence.helmet_index, 
            chest_index: appearence.chest_index, 
            shorts_index: appearence.shorts_index, 
            gloves_index: appearence.gloves_index, 
            boots_index: appearence.boots_index, 
            unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
            face: appearence.face, 
            hair: appearence.hair, 
            unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
        }
    }
}

impl From<&mut Packet> for PlayerAppearResponse {
    fn from(packet: &mut Packet) -> Self {
        let player_id = packet.get_u32();
        let name = packet.get_string();
        let class = packet.get_u8();
        let (class, is_current_player) = match class {
            0 => (PlayerClass::Knight, false),
            1 => (PlayerClass::Mage, false),
            2 => (PlayerClass::Archer, false),
            128 => (PlayerClass::Knight, true),
            129 => (PlayerClass::Mage, true),
            _ => (PlayerClass::Archer, true)
        };
        let x = packet.get_u32();
        let y = packet.get_u32();
        let z = packet.get_u32();
        let unknown1 = packet.get_buffer(10);
        let weapon_index = packet.get_u16();
        let shield_index = packet.get_u16();
        let helmet_index = packet.get_u16();
        let chest_index = packet.get_u16();
        let shorts_index = packet.get_u16();
        let gloves_index = packet.get_u16();
        let boots_index = packet.get_u16();
        let unknown2 = packet.get_buffer(62);
        let face = packet.get_u8();
        let hair = packet.get_u8();
        let unknown3 = packet.get_buffer(54);
        PlayerAppearResponse { player_id, name, x, y, z, unknown1, helmet_index, chest_index, shorts_index, gloves_index, boots_index, unknown2, face, hair, unknown3, weapon_index, shield_index, class, is_current_player }
    }
}

impl From<&PlayerAppearResponse> for Packet {
    fn from(val: &PlayerAppearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.player_id);
        packet.write_string(&val.name);
        if val.is_current_player {
            match val.class {
                PlayerClass::Knight => packet.write_u8(128),
                PlayerClass::Mage => packet.write_u8(129),
                PlayerClass::Archer => packet.write_u8(130),
            };
        } else {
            match val.class {
                PlayerClass::Knight => packet.write_u8(0),
                PlayerClass::Mage => packet.write_u8(1),
                PlayerClass::Archer => packet.write_u8(2),
            };
        }
        packet.write_u32(val.x);
        packet.write_u32(val.y);
        packet.write_u32(val.z);
        packet.write_buffer(&val.unknown1);
        packet.write_u16(val.weapon_index);
        packet.write_u16(val.shield_index);
        packet.write_u16(val.helmet_index);
        packet.write_u16(val.chest_index);
        packet.write_u16(val.shorts_index);
        packet.write_u16(val.gloves_index);
        packet.write_u16(val.boots_index);
        packet.write_buffer(&val.unknown2);
        packet.write_u8(val.face);
        packet.write_u8(val.hair);
        packet.write_buffer(&val.unknown3);
        packet
    }
}