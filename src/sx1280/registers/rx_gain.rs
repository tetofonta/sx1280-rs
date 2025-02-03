use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::registers::{SX1280Register, SX1280RegisterError};
use crate::sx1280::SX1280Mode;

#[derive(Clone, Copy, Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum RxGainSensitivity {
    HighSensitivity = 3,
    LowSensitivity = 0,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[bitfield(u8, defmt=true)]
pub struct RxGain{
    #[bits(6)] _unknown: u8,
    #[bits(2)] sensitivity: RxGainSensitivity,
}


impl TryFrom<[u8; 1]> for RxGain {
    type Error = SX1280RegisterError;

    fn try_from(value: [u8; 1]) -> Result<Self, Self::Error> {
        Ok(Self::from_bits(value[0]))
    }
}

impl<MODE: SX1280Mode> SX1280Register<MODE> for RxGain {
    const ADDRESS: u16 = 0x891;
    type BufferType = [u8; 1];
    fn as_write_bytes(&self) -> Self::BufferType {
        [self.into_bits()]
    }
}