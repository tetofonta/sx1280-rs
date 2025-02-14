use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::sx1280::registers::{SX1280Register, SX1280RegisterError};
use crate::sx1280::SX1280Mode;

#[derive(Clone, Copy, Debug, Format)]
pub struct FrequencyCompensationMode(pub u8);

impl TryFrom<[u8; 1]> for FrequencyCompensationMode {
    type Error = SX1280RegisterError;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(Self(value[0]))
    }
}

impl<MODE: SX1280Mode> SX1280Register<MODE> for FrequencyCompensationMode {
    const ADDRESS: u16 = 0x93C;
    type BufferType = [u8; 1];
    fn as_write_bytes(&self) -> Self::BufferType {
        [self.0]
    }
}