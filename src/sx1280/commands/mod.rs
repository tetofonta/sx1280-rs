pub mod get_status;
pub mod set_packet_type;
pub mod set_sleep;
pub mod set_standby;

pub mod set_fs;
pub mod set_tx;
pub mod set_rx;
pub mod set_rx_dc;
pub mod set_long_preamble;
pub mod set_cad;
pub mod set_rf_frequency;
pub mod set_tx_parameters;
pub mod set_cad_parameters;
pub mod set_buffer_base_address;
pub mod set_modulation_parameters;
pub mod set_packet_parameters;
pub mod get_rx_buffer_status;
pub mod get_packet_status;

use core::error::Error;
use core::fmt::{Display, Formatter};
use bitfield_struct::{FromBits, IntoBits};
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive};
use crate::sx1280::SX1280Mode;


#[derive(Clone, Copy, Debug, Format)]
pub enum SX1280CommandError{
    InvalidResponse,
    InvalidArgument,
    Other
}

impl Display for SX1280CommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "Command Error")
    }
}

impl Error for SX1280CommandError {}

pub trait SX1280Command<MODE: SX1280Mode> {
    const OPCODE: u8;

    type ArgumentsBufferType: AsRef<[u8]> + AsMut<[u8]>;
    type ResponseBufferType: AsRef<[u8]> + AsMut<[u8]> + Default;

    type ResponseType: Sized + TryFrom<(u8, Self::ResponseBufferType), Error = SX1280CommandError>;
    fn as_write_bytes(&self) -> Result<Self::ArgumentsBufferType, SX1280CommandError>;
}


pub struct NullResponse;
pub type NullResponseBufferType = [u8; 0];
pub type NullArgumentsBufferType = [u8; 0];
impl TryFrom<(u8, [u8; 0])> for NullResponse {
    type Error = SX1280CommandError;

    fn try_from(_value: (u8, [u8; 0])) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}


#[derive(Clone, Copy, Debug, Format, FromPrimitive, IntoPrimitive, IntoBits, FromBits)]
#[repr(u8)]
pub enum PeriodBase {
    Base15u625s = 0,
    Base62u5s = 1,
    Base1ms = 2,
    Base4ms = 3,

    #[num_enum(catch_all)]
    Unknown(u8),
}


//FIXME: implement
//  SetTxContinuousWave,
//  SetTxContinuousPreamble,
//  SetAutoTx,
//  SetAutoRx,
//  GetPacketType,
//  Non LoRa SetModulationParams
//  Non LoRa SetPacketParameters
//  Non LoRa GetPacketStatus
//  GetRssiInst
//  All the irqs

//TODO: implement a set_mode enum for all the possible modes with a into method