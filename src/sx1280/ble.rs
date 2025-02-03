use crate::sx1280::{SX1280Mode, SX1280ModeValid};
use crate::sx1280::commands::set_packet_type::PacketType;

pub struct ModeBLE;

impl SX1280Mode for ModeBLE {}
impl SX1280ModeValid for ModeBLE {
    const PACKET_CONST: PacketType = PacketType::BLE;
}