use bitfield_struct::{bitfield};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::gfsk::ModeGFSK;
use crate::sx1280::lora::ModeLoRa;

pub struct SetLongPreambleModeCommand(pub bool);

impl SX1280Command<ModeGFSK> for SetLongPreambleModeCommand {
    const OPCODE: u8 = 0x9B;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([if self.0 { 1 } else { 0 }])
    }
}

impl SX1280Command<ModeLoRa> for SetLongPreambleModeCommand {
    const OPCODE: u8 = 0x9B;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([if self.0 { 1 } else { 0 }])
    }
}
