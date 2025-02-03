use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;


pub struct SetRFFrequencyCommand(u32);


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetRFFrequencyCommand {
    const OPCODE: u8 = 0x86;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        if self.0 > 0xFFFFFF { return Err(SX1280CommandError::InvalidArgument)}
        Ok(self.0.to_be_bytes()[1..].try_into().unwrap())
    }
}
