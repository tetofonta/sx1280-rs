use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

#[derive(Clone, Copy, Debug, Format, IntoPrimitive, FromPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum CadSymbolsNumber {
    Cad1Symbol = 0,
    Cad2Symbols = 0x20,
    Cad4Symbols = 0x40,
    Cad8Symbols = 0x60,
    Cad16Symbols = 0x80,

    #[num_enum(catch_all)]
    Unknown(u8),
}

#[bitfield(u8, defmt=true)]
pub struct SetCadParametersCommand{
    #[bits(8)] ramp: CadSymbolsNumber,
}

impl SX1280Command<ModeLoRa> for SetCadParametersCommand {
    const OPCODE: u8 = 0x88;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.into_bits()])
    }
}
