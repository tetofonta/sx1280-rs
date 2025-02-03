use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;


pub struct SetRFFrequencyCommand<T: IntoRFFrequency>(pub T);

pub trait IntoRFFrequency: Sized + Copy + Clone{
    fn into_rf_frequency(self) -> u32;
}
impl IntoRFFrequency for u32 {
    fn into_rf_frequency(self) -> u32 {
        const XTAL_FREQ: u64 = 52_000_000;
        // F_out = F_set * XTAL_FREQ/(1 << 18)
        // F_out / F_set = XTAL_FREQ/(1 << 18)
        // 1 / F_set = XTAL_FREQ/((1 << 18) * F_out)
        // F_set = (F_out << 18)/XTAL_FREQ
        let x = (self as u64) << 18;
        (x / XTAL_FREQ) as u32
    }
}


impl<MODE: SX1280ModeValid, T: IntoRFFrequency> SX1280Command<MODE> for SetRFFrequencyCommand<T> {
    const OPCODE: u8 = 0x86;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        let v = self.0.clone().into_rf_frequency();
        if v > 0xFFFFFF { return Err(SX1280CommandError::InvalidArgument)}
        Ok(v.to_be_bytes()[1..].try_into().unwrap())
    }
}
