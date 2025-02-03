use bitfield_struct::{bitfield};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

#[bitfield(u8, defmt=true)]
pub struct SetSleepModeCommand {
    retain_ram: bool,
    retain_data_buffer: bool,
    #[bits(6)] _unused: u8
}

impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetSleepModeCommand {
    const OPCODE: u8 = 0x84;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.into_bits()])
    }
}
