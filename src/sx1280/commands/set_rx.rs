use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

#[derive(Clone, Copy, Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u32)]
pub enum RxPeriod {
    OneShot = 0,
    Infinite = 65535,
    #[num_enum(catch_all)]
    Timeout(u32),
}

pub struct SetRxModeCommand {
    pub period: RxPeriod,
    pub period_base: PeriodBase,
}


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetRxModeCommand {
    const OPCODE: u8 = 0x82;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.period_base as u8, ((self.period.into_bits()) >> 8) as u8, ((self.period.into_bits()) & 255) as u8])
    }
}
