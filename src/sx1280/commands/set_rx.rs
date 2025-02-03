use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;
use endian_num::be32;

#[derive(Clone, Copy, Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u32)]
pub enum RxPeriod {
    OneShot = 0,
    Infinite = 65535,
    #[num_enum(catch_all)]
    Timeout(u32),
}

#[bitfield(u32, defmt=true, repr = be32, from = be32::from_ne, into = be32::to_ne)]
pub struct SetRxModeCommand {
    _padding: u8,
    #[bits(16)] period: RxPeriod,
    #[bits(8)] period_base: PeriodBase,
}


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetRxModeCommand {
    const OPCODE: u8 = 0x82;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        match self.period_base() {
            PeriodBase::Unknown(_) => Err(SX1280CommandError::InvalidArgument),
            x => Ok(self.into_bits().to_be_bytes()[0..3].try_into().unwrap())
        }
    }
}
