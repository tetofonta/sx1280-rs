use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullArgumentsBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280Mode;

pub struct GetRxBufferStatusCommand;

#[bitfield(u16, defmt=true)]
pub struct BufferStatus {
    #[bits(8)]
    rx_buffer_start_pointer: u8,
    #[bits(8)]
    rx_payload_len: u8,
}

impl<MODE: SX1280Mode> SX1280Command<MODE> for GetRxBufferStatusCommand {
    const OPCODE: u8 = 0x17;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = [u8; 3];
    type ResponseType = BufferStatus;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([0; 0])
    }
}

impl TryFrom<(u8, [u8; 3])> for BufferStatus {
    type Error = SX1280CommandError;

    fn try_from(value: (u8, [u8; 3])) -> Result<Self, Self::Error> {
        Ok(Self::from_bits(u16::from_be_bytes(value.1[1..].try_into().unwrap())))
    }
}