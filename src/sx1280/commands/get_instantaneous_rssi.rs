use core::ops::Deref;
use crate::sx1280::commands::{NullArgumentsBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

pub struct GetInstantaneousRssiCommand;

pub struct InstantaneousRssi(f32);
impl Deref for InstantaneousRssi {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


impl SX1280Command<ModeLoRa> for GetInstantaneousRssiCommand {
    const OPCODE: u8 = 0x1F;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = [u8; 2];
    type ResponseType = InstantaneousRssi;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([0; 0])
    }
}

impl TryFrom<(u8, [u8; 2])> for InstantaneousRssi {
    type Error = SX1280CommandError;
    fn try_from(value: (u8, [u8; 2])) -> Result<Self, Self::Error> {
        Ok(Self(-(value.0 as f32) / 2.0f32))
    }
}