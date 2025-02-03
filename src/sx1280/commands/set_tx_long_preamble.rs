use crate::sx1280::commands::{NullArgumentsBufferType, NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;
use crate::sx1280::{SX1280Mode, SX1280ModeValid};

pub struct SetTXLongPreambleCommand;

impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetTXLongPreambleCommand {
    const OPCODE: u8 = 0xD2;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([])
    }
}
