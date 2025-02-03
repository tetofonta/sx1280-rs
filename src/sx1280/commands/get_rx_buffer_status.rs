use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullArgumentsBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280Mode;

pub struct GetRxBufferStatusCommand;

pub struct BufferStatus {
    pub rx_buffer_start_pointer: u8,
    pub rx_payload_len: u8,
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
        Ok(Self {
            rx_buffer_start_pointer: value.1[2],
            rx_payload_len: value.1[1],
        })
    }
}