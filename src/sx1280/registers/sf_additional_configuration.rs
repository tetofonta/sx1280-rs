use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::TryFromPrimitive;
use crate::sx1280::registers::{SX1280Register, SX1280RegisterError};
use crate::sx1280::{SX1280Error, SX1280Mode};

#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum SFAdditionalConfiguration {
    SF5_6 = 0x1E,
    SF7_8 = 0x37,
    SFOther = 0x32,
}

impl TryFrom<[u8; 1]> for SFAdditionalConfiguration {
    type Error = SX1280RegisterError;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        SFAdditionalConfiguration::try_from(value[0]).map_err(|x| SX1280RegisterError::Invalid)
    }
}

impl<MODE: SX1280Mode> SX1280Register<MODE> for SFAdditionalConfiguration {
    const ADDRESS: u16 = 0x925;
    type BufferType = [u8; 1];
    fn as_write_bytes(&self) -> Self::BufferType {
        [*self as u8]
    }
}