use defmt::Format;
use num_enum_derive::{IntoPrimitive, TryFromPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280Mode;

#[derive(Clone, Copy, Format, TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
pub enum PacketType {
    GFSK = 0,
    LoRa = 1,
    #[cfg(feature = "ranging")]
    Ranging = 2,
    FLRC = 3,
    BLE = 4,
}

pub struct SetPacketTypeCommand(pub PacketType);

impl<MODE: SX1280Mode> SX1280Command<MODE> for SetPacketTypeCommand {
    const OPCODE: u8 = 0x8A;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.0.into()])
    }
}
