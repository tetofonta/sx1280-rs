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
pub mod set_tx_continuous_wave;
pub mod set_tx_long_preamble;
pub mod get_instantaneous_rssi;
pub mod set_irq_params;
pub mod get_irq_status;
pub mod clear_irq;

use core::error::Error;
use core::fmt::{Display, Formatter};
use bitfield_struct::{FromBits, IntoBits};
use bitflags::bitflags;
use defmt::Format;
use num_enum_derive::{FromPrimitive, IntoPrimitive, TryFromPrimitive};
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


#[derive(Clone, Copy, Debug, Format, TryFromPrimitive)]
#[repr(u8)]
pub enum PeriodBase {
    Base15u625s = 0,
    Base62u5s = 1,
    Base1ms = 2,
    Base4ms = 3,
}

#[derive(Clone, Copy, Debug, Format)]
pub struct SX1280Interrupt(u16);

bitflags! {
    impl SX1280Interrupt: u16 {
        const TxDone =                      0x0001;
        const RxDone =                      0x0002;
        const SyncWordValid =               0x0004;
        const SyncWordError =               0x0008;
        const HeaderValid =                 0x0010;
        const HeaderError =                 0x0020;
        const CRCError =                    0x0040;
        const RangingSlaveResponseDone =    0x0080;
        const RangingSlaveRequestDiscard =  0x0100;
        const RangingMasterResultValid =    0x0200;
        const RangingMasterTimeout =        0x0400;
        const RangingSlaveRequestValid =    0x0800;
        const CADDone =                     0x1000;
        const CADDetect =                   0x2000;
        const RXTXTimeout =                 0x4000;
        const PreambleDetect =              0x8000;
        const _ = !0;
    }
}


//FIXME: implement
//  SetAutoTx,
//  SetAutoRx,
//  GetPacketType,
//  Non LoRa SetModulationParams
//  Non LoRa SetPacketParameters
//  Non LoRa GetPacketStatus

//TODO: implement a set_mode enum for all the possible modes with a into method