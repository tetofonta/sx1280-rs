use crate::sx1280::commands::{NullArgumentsBufferType, NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

pub struct SetCAD;

impl SX1280Command<ModeLoRa> for SetCAD {
    const OPCODE: u8 = 0xC5;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([])
    }
}
