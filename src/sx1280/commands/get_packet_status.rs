use crate::sx1280::commands::{NullArgumentsBufferType, SX1280Command, SX1280CommandError};
use crate::sx1280::lora::ModeLoRa;

pub struct GetPacketStatusCommand;

pub struct LoRaPacketStatus {
    pub rssi: f32,
    pub snr: f32,
}

impl SX1280Command<ModeLoRa> for GetPacketStatusCommand {
    const OPCODE: u8 = 0x1D;
    type ArgumentsBufferType = NullArgumentsBufferType;
    type ResponseBufferType = [u8; 6];
    type ResponseType = LoRaPacketStatus;

    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError> {
        Ok([0; 0])
    }
}

impl TryFrom<(u8, [u8; 6])> for LoRaPacketStatus {
    type Error = SX1280CommandError;

    fn try_from(value: (u8, [u8; 6])) -> Result<Self, Self::Error> {
        let rssi = -(value.1[1] as f32) / 2.0f32;
        let snr = (value.1[2] as i8) as f32 / 4.0f32;
        if snr <= 0.0 {
            Ok(Self{
                rssi: rssi - snr,
                snr
            })
        } else {
            Ok(Self{
                rssi,
                snr
            })
        }
    }
}