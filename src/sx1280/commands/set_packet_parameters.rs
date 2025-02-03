use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum IQMode {
    Standard = 0x40,
    Inverted = 0x00,

    #[num_enum(catch_all)]
    Unknown(u8),
}
#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum CrcMode {
    Enabled = 0x20,
    Disabled = 0x00,

    #[num_enum(catch_all)]
    Unknown(u8),
}
#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum HeaderType {
    Implicit = 0x80,
    Explicit = 0x00,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[bitfield(u64, defmt=true)]
pub struct SetLoraModulationParameters{
    _pad: u8,
    _pad: u16,
    #[bits(8)] iq_mode: IQMode,
    #[bits(8)] crc_mode: CrcMode,
    #[bits(8)] payload_length: u8,
    #[bits(8)] header_type: HeaderType,
    #[bits(4)] preamble_len_mantissa: u8,
    #[bits(4)] preamble_len_exponent: u8
}

impl SX1280Command<ModeLoRa> for SetLoraModulationParameters {
    const OPCODE: u8 = 0x8C;
    type ArgumentsBufferType = [u8; 7];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok(self.into_bits().to_be_bytes()[0..7].try_into().unwrap())
    }
}


