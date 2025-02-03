use crate::sx1280::{SX1280Mode, SX1280ModeValid};
use crate::sx1280::commands::set_packet_type::PacketType;

pub struct ModeLoRaRanging;

impl SX1280Mode for ModeLoRaRanging {}
impl SX1280ModeValid for ModeLoRaRanging {
    const PACKET_CONST: PacketType = PacketType::Ranging;
}