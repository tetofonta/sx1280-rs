use bitfield_struct::bitfield;
use crate::sx1280::commands::{NullResponse, NullResponseBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::SX1280ModeValid;

#[bitfield(u16, defmt=true)]
pub struct SetBufferBaseAddressCommand{
    rx_base_address: u8,
    tx_base_address: u8,
}

impl<MODE: SX1280ModeValid> SX1280Command<MODE> for SetBufferBaseAddressCommand {
    const OPCODE: u8 = 0x8F;
    type ArgumentsBufferType = [u8; 2];
    type ResponseBufferType = NullResponseBufferType;
    type ResponseType = NullResponse;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok(self.into_bits().to_be_bytes())
    }
}
