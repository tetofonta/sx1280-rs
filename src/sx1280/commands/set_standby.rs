use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum StandbyMode {
    StandbyRC = 0,
    StandbyXOSC = 1,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[bitfield(u8, defmt=true)]
pub struct SetStandbyModeCommand{
    #[bits(8)] mode: StandbyMode,
}

impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetStandbyModeCommand {
    const OPCODE: u8 = 0x80;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        match self.mode() {
            StandbyMode::Unknown(_) => Err(SX1280CommandError::InvalidArgument),
            x => Ok([self.into_bits()])
        }
    }
}
