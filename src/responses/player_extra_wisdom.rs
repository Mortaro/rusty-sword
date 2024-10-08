use crate::components::current_magic_points::CurrentMagicPoints;
use crate::components::maximum_magic_points::MaximumMagicPoints;
use crate::framework::packet::Packet;
use crate::components::final_points::FinalPoints;
use crate::components::magical_attack::MagicalAttack;
use crate::components::extra_points::ExtraPoints;

pub const HEADER: u8 = 69;
pub const SUB_HEADER: u8 = 46;

#[derive(Debug)]
pub struct PlayerExtraWisdomResponse {
    pub extra_wisdom: u16,
    pub current_magic_points: u16,
    pub maximum_magic_points: u16,
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
    pub curse_resistence: u16,
}

impl PlayerExtraWisdomResponse {
    pub fn new(
        extra_points: &ExtraPoints, 
        current_magic_points: &CurrentMagicPoints, 
        maximum_magic_points: &MaximumMagicPoints,
        magical_attack: &MagicalAttack, 
        final_points: &FinalPoints
    ) -> Self {
        PlayerExtraWisdomResponse { 
            extra_wisdom: extra_points.extra_wisdom, 
            current_magic_points: current_magic_points.current_magic_points, 
            maximum_magic_points: maximum_magic_points.maximum_magic_points, 
            minimum_magical_attack: magical_attack.minimum_magical_attack, 
            maximum_magical_attack: magical_attack.maximum_magical_attack, 
            curse_resistence: final_points.curse_resistence 
        }
    }
}

impl From<&mut Packet> for PlayerExtraWisdomResponse {
    fn from(packet: &mut Packet) -> Self {
        let extra_wisdom = packet.get_u16();
        let current_magic_points = packet.get_u16();
        let maximum_magic_points = packet.get_u16();
        let minimum_magical_attack = packet.get_u16();
        let maximum_magical_attack = packet.get_u16();
        let curse_resistence = packet.get_u16();
        PlayerExtraWisdomResponse { extra_wisdom, current_magic_points, maximum_magic_points, minimum_magical_attack, maximum_magical_attack, curse_resistence  }
    }
}

impl From<&PlayerExtraWisdomResponse> for Packet {
    fn from(val: &PlayerExtraWisdomResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_u16(val.extra_wisdom);
        packet.write_u16(val.current_magic_points);
        packet.write_u16(val.maximum_magic_points);
        packet.write_u16(val.minimum_magical_attack);
        packet.write_u16(val.maximum_magical_attack);
        packet.write_u16(val.curse_resistence);
        packet
    }
}