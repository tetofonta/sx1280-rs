use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum LoRaIQMode {
    Standard = 0x40,
    Inverted = 0x00,
}
#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum LoRaCrcMode {
    Enabled = 0x20,
    Disabled = 0x00,
}
#[derive(Clone, Copy, Debug, Format, TryFromPrimitive, PartialEq, Eq)]
#[repr(u8)]
pub enum LoRaHeaderType {
    Implicit = 0x80,
    Explicit = 0x00,
}

#[bitfield(u8, defmt=true)]
pub struct LoRaPreambleLength {
    #[bits(4)] mantissa: u8,
    #[bits(4)] exponent: u8
}
impl From<LoRaPreambleLength> for u32 {
    fn from(value: LoRaPreambleLength) -> Self {
        (1u32 << value.exponent()) * value.mantissa() as u32
    }
}
impl TryFrom<u32> for LoRaPreambleLength {
    type Error = SX1280CommandError;

    fn try_from(mut value: u32) -> Result<Self, Self::Error> {
        let mut exp = 0;
        if value == 0 { return Ok(Self::new().with_exponent(0).with_mantissa(0)) }
        while (value & 1) == 0 {
            exp += 1;
            value = value >> 1;
        }
        if exp > 15 || value > 15 { return Err(SX1280CommandError::InvalidArgument) }
        Ok(Self::new().with_exponent(exp).with_mantissa(value as u8))
    }
}

pub struct SetLoraPacketParameters{
    pub iq_mode: LoRaIQMode,
    pub crc_mode: LoRaCrcMode,
    pub payload_length: u8,
    pub header_type: LoRaHeaderType,
    pub preamble_length: LoRaPreambleLength
}

impl SX1280Command<ModeLoRa> for SetLoraPacketParameters {
    const OPCODE: u8 = 0x8C;
    type ArgumentsBufferType = [u8; 7];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        if self.header_type.eq(&LoRaHeaderType::Explicit) && self.payload_length > 253 {
            return Err(SX1280CommandError::InvalidArgument)
        }
        Ok([
            self.preamble_length.into_bits(),
            self.header_type as u8,
            self.payload_length as u8,
            self.crc_mode as u8,
            self.iq_mode as u8,
            0,
            0
        ])
    }
}


