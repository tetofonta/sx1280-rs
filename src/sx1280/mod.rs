pub mod registers;
pub mod commands;
pub mod common;
pub mod uninitialized;
pub mod lora;
pub mod gfsk;
pub mod flrc;
pub mod ble;
#[cfg(feature = "ranging")]
pub mod ranging;

use core::marker::PhantomData;
use defmt::Format;
use embedded_hal::digital::{ErrorType as PinErrorType, Error as PinError, InputPin, OutputPin};
use embedded_hal::spi::{Error as SpiErrorType, SpiBus};
use crate::spi::{SpiDevice, SpiError};
use crate::sx1280::commands::set_packet_type::PacketType;
use crate::sx1280::commands::SX1280CommandError;
use crate::sx1280::registers::SX1280RegisterError;

#[derive(Debug, Format)]
pub enum SX1280Error<DEV: SXDevice> {
    SpiTransactionError(SpiError<DEV::CSType>),
    SpiError(DEV::SpiError),
    CommandError(SX1280CommandError),
    Busy,
    PinError,
    RegisterError(SX1280RegisterError),
    Timeout,
    Other,
}

pub type SX1280Result<T, DEV> = Result<T, SX1280Error<DEV>>;

pub trait SX1280Mode{
}
pub trait SX1280ModeValid : SX1280Mode {
    const PACKET_CONST: PacketType;
}

pub trait SXDevice {
    type CSType: PinErrorType;
    type SpiError: SpiErrorType;
}

pub struct SX1280<'a, SPI: SpiBus<u8>, CS: OutputPin, BUSY: InputPin, RESET: OutputPin, MODE: SX1280Mode> {
    spi: SpiDevice<'a, SPI, CS>,
    busy_pin: BUSY,
    reset_pin: RESET,
    _phantom: PhantomData<MODE>,
}
impl<'a, SPI: SpiBus<u8>, CS: OutputPin, BUSY: InputPin, RESET: OutputPin, MODE: SX1280Mode> SXDevice for SX1280<'a, SPI, CS, BUSY, RESET, MODE> {
    type CSType = CS;
    type SpiError = SPI::Error;
}


impl<DEV, ERR> From<ERR> for SX1280Error<DEV>
where
    ERR: PinError,
    DEV: SXDevice,
{
    fn from(_: ERR) -> Self {
        SX1280Error::PinError
    }
}


impl<DEV> From<SpiError<DEV::CSType>> for SX1280Error<DEV>
where
    DEV: SXDevice,
{
    fn from(value: SpiError<DEV::CSType>) -> Self {
        SX1280Error::SpiTransactionError(value)
    }
}

impl<DEV: SXDevice> From<SX1280RegisterError> for SX1280Error<DEV> {
    fn from(value: SX1280RegisterError) -> Self {
        Self::RegisterError(value)
    }
}

impl<DEV: SXDevice> From<SX1280CommandError> for SX1280Error<DEV> {
    fn from(value: SX1280CommandError) -> Self {
        Self::CommandError(value)
    }
}