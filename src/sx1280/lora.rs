use crate::sx1280::{SX1280Mode, SX1280ModeValid};
use crate::sx1280::commands::set_packet_type::PacketType;

pub struct ModeLoRa;

impl SX1280Mode for ModeLoRa {}
impl SX1280ModeValid for ModeLoRa {
    const PACKET_CONST: PacketType = PacketType::LoRa;
}