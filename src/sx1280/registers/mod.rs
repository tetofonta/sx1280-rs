pub mod rx_gain;
pub mod sf_additional_configuration;
pub mod frequency_compensation_mode;

use core::error::Error;
use core::fmt::{Display, Formatter};
use defmt::Format;
use crate::sx1280::SX1280Mode;


#[derive(Clone, Copy, Debug, Format)]
pub enum SX1280RegisterError{
    NotEnoughData,
    Invalid,
    Other
}

impl Display for SX1280RegisterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Register Error")
    }
}

impl Error for SX1280RegisterError {}



pub trait SX1280Register<MODE: SX1280Mode>: TryFrom<Self::BufferType, Error = SX1280RegisterError>{
    const ADDRESS: u16;

    type BufferType: AsMut<[u8]> + AsRef<[u8]> + Default;

    fn as_write_bytes(&self) -> Self::BufferType;
}

