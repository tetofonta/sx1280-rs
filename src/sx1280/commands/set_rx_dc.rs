use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, PeriodBase, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

pub struct SetRxDCModeCommand {
    pub sleep_period: u16,
    pub rx_period: u16,
    pub period_base: PeriodBase,
}


impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetRxDCModeCommand {
    const OPCODE: u8 = 0x94;
    type ArgumentsBufferType = [u8; 5];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([
            self.period_base as u8,
            ((self.rx_period) >> 8) as u8,
            ((self.rx_period) & 255) as u8,
            ((self.sleep_period) >> 8) as u8,
            ((self.sleep_period) & 255) as u8
        ])

    }
}
