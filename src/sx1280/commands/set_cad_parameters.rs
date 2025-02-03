use bitfield_struct::{bitfield, FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum CadSymbolsNumber {
    Cad1Symbol = 0,
    Cad2Symbols = 0x20,
    Cad4Symbols = 0x40,
    Cad8Symbols = 0x60,
    Cad16Symbols = 0x80,
}

pub struct SetCadParametersCommand{
    pub symbol_number: CadSymbolsNumber,
}

impl SX1280Command<ModeLoRa> for SetCadParametersCommand {
    const OPCODE: u8 = 0x88;
    type ArgumentsBufferType = [u8; 1];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([self.symbol_number as u8])
    }
}
