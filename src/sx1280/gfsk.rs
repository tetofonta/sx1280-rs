use crate::sx1280::{SX1280Mode, SX1280ModeValid};
use crate::sx1280::commands::set_packet_type::PacketType;

pub struct ModeGFSK;

impl SX1280Mode for ModeGFSK {}
impl SX1280ModeValid for ModeGFSK {
    const PACKET_CONST: PacketType = PacketType::GFSK;
}