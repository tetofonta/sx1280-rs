use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum SpreadingFactor {
    SF5 = 0x50,
    SF6 = 0x60,
    SF7 = 0x70,
    SF8 = 0x80,
    SF9 = 0x90,
    SF10 = 0xA0,
    SF11 = 0xB0,
    SF12 = 0xC0,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum Bandwidth {
    BW1M625Hz = 0x0A,
    BW812k5Hz = 0x18,
    BW406k25Hz = 0x26,
    BW203k125Hz = 0x34,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum CodingRate {
    CR4_5 = 1,
    CR4_6 = 2,
    CR4_7 = 3,
    CR4_8 = 4,
    CR4_5Alt = 5,
    CR4_6Alt = 6,
    CR4_8Alt = 7,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[bitfield(u32, defmt=true)]
pub struct SetLoraModulationParameters{
    _pad: u8,
    #[bits(8)] coding_rate: CodingRate,
    #[bits(8)] bandwidth: Bandwidth,
    #[bits(8)] spreading_factor: SpreadingFactor,
}

impl SX1280Command<ModeLoRa> for SetLoraModulationParameters {
    const OPCODE: u8 = 0x8E;
    type ArgumentsBufferType = [u8; 3];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok(self.into_bits().to_be_bytes()[0..3].try_into().unwrap())
    }
}


