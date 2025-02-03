use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError, SX1280Interrupt};
use crate::sx1280::SX1280ModeValid;

pub struct SetIRQParametersCommand {
    pub irq_mask: SX1280Interrupt,
    pub dio_mask: [SX1280Interrupt; 3]
}


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetIRQParametersCommand {
    const OPCODE: u8 = 0x8D;
    type ArgumentsBufferType = [u8; 8];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([
            (self.irq_mask.bits() >> 8) as u8,
            (self.irq_mask.bits() & 255) as u8,
            (self.dio_mask[0].bits() >> 8) as u8,
            (self.dio_mask[0].bits() & 255) as u8,
            (self.dio_mask[1].bits() >> 8) as u8,
            (self.dio_mask[1].bits() & 255) as u8,
            (self.dio_mask[2].bits() >> 8) as u8,
            (self.dio_mask[2].bits() & 255) as u8,
        ])
    }
}
