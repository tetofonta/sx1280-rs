use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullArgumentsBufferType, SX1280Command, SX1280CommandError, SX1280Interrupt};
use crate::sx1280::SX1280Mode;

pub struct GetIrqStatusCommand;

impl<MODE: SX1280Mode> SX1280Command<MODE> for GetIrqStatusCommand {
    const OPCODE: u8 = 0x15;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = [u8; 3];
    type ResponseType = SX1280Interrupt;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([0; 0])
    }
}

impl TryFrom<(u8, [u8; 3])> for SX1280Interrupt {
    type Error = SX1280CommandError;

    fn try_from(value: (u8, [u8; 3])) -> Result<Self, Self::Error> {
        Ok(SX1280Interrupt::from_bits(u16::from_be_bytes(value.1[1..].try_into().unwrap())).ok_or(SX1280CommandError::InvalidResponse)?)
    }
}