use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;
use endian_num::be64;

#[bitfield(u64, defmt=true, repr = be64, from = be64::from_ne, into = be64::to_ne)]
pub struct SetRxDCModeCommand {
    _padding: u16,
    _padding: u8,
    sleep_period: u16,
    rx_period: u16,
    #[bits(8)] period_base: PeriodBase,
}


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetRxDCModeCommand {
    const OPCODE: u8 = 0x94;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        match self.period_base() {
            PeriodBase::Unknown(_) => Err(SX1280CommandError::InvalidArgument),
            _ => Ok(self.into_bits().to_be_bytes()[0..5].try_into().unwrap())
        }
    }
}
