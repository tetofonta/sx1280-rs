use defmt::Format;
use crate::sx1280::registers::{SX1280Register, SX1280RegisterError};
use crate::sx1280::SX1280Mode;

#[derive(Clone, Copy, Debug, Format)]
pub struct RxGain(pub u8);

impl TryFrom<[u8; 1]> for RxGain {
    type Error = SX1280RegisterError;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(Self(value[0]))
    }
}

impl<MODE: SX1280Mode> SX1280Register<MODE> for RxGain {
    const ADDRESS: u16 = 0x891;
    type BufferType = [u8; 1];
    fn as_write_bytes(&self) -> Self::BufferType {
        self.0.to_be_bytes()
    }
}