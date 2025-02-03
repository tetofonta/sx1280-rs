use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError, SX1280Interrupt};
use crate::sx1280::SX1280ModeValid;

pub struct ClearIrqCommand(pub SX1280Interrupt);


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for ClearIrqCommand {
    const OPCODE: u8 = 0x97;
    type ArgumentsBufferType = [u8; 2];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([
            (self.0.bits() >> 8) as u8,
            (self.0.bits() & 255) as u8
        ])
    }
}
