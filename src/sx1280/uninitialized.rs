use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use crate::spi::SpiDevice;
use crate::sx1280::{SX1280Mode, SX1280Result, SX1280};
use core::marker::PhantomData;

pub struct ModeUninitialized;
impl SX1280Mode for ModeUninitialized{
}

impl<'a, SPI: SpiBus<u8>, CS: OutputPin, BUSY: InputPin, RESET: OutputPin> SX1280<'a, SPI, CS, BUSY, RESET, ModeUninitialized> {
    pub fn new(spi: SpiDevice<'a, SPI, CS>, busy: BUSY, mut reset: RESET) -> SX1280Result<Self, Self> {
        reset.set_high()?;
        Ok(Self{
            spi,
            busy_pin: busy,
            reset_pin: reset,
            _phantom: PhantomData { },
        })
    }
}