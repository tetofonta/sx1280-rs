use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::{SX1280Mode, SX1280ModeValid};

#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum StandbyMode {
    StandbyRC = 0,
    StandbyXOSC = 1,
}

pub struct SetStandbyModeCommand{
    pub mode: StandbyMode,
}

impl<MODE: SX1280Mode> SX1280Command<MODE> for SetStandbyModeCommand {
    const OPCODE: u8 = 0x80;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.mode as u8])
    }
}
