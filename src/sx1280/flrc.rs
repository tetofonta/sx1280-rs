use crate::sx1280::{SX1280Mode, SX1280ModeValid};
use crate::sx1280::commands::set_packet_type::PacketType;

pub struct ModeFLRC;

impl SX1280Mode for ModeFLRC {}
impl SX1280ModeValid for ModeFLRC {
    const PACKET_CONST: PacketType = PacketType::FLRC;
}