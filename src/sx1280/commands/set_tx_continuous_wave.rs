use crate::sx1280::commands::{NullArgumentsBufferType, NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;
use crate::sx1280::SX1280Mode;

pub struct SetTXContinuousWaveCommand;

impl<MODE: SX1280Mode> SX1280Command<MODE> for SetTXContinuousWaveCommand {
    const OPCODE: u8 = 0xD1;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([])
    }
}
