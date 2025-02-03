use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum TxRampTime {
    Ramp2us = 0,
    Ramp4us = 0x20,
    Ramp6us = 0x40,
    Ramp8us = 0x60,
    Ramp10us = 0x80,
    Ramp12us = 0xA0,
    Ramp16us = 0xC0,
    Ramp20us = 0xE0,
}

pub struct SetTxParametersCommand{
    pub ramp: TxRampTime,
    pub power: i8
}

impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetTxParametersCommand {
    const OPCODE: u8 = 0x8E;
    type ArgumentsBufferType = [u8; 2];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        if self.power < -18 || self.power > 13 { return Err(SX1280CommandError::InvalidArgument) }
        Ok([(self.power + 18) as u8, self.ramp as u8])
    }
}
