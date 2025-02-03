use defmt::Format;
use embedded_hal::digital::{ErrorType as PinErrorType, OutputPin};
use embedded_hal::spi::{ErrorType as SpiErrorType, SpiBus};
use rp2040_hal::fugit::ExtU64;
use rtic_monotonics::Monotonic;
use crate::Mono;

#[derive(Debug, Format, Clone, Eq, PartialEq)]
pub enum SpiError<CS: PinErrorType> {
    PinError(CS::Error),
    AlreadyStarted,
}

pub type SpiResult<T, CS> = Result<T, SpiError<CS>>;



pub struct SpiDevice<'a, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    spi: &'a mut SPI,
    cs: Option<CS>
}


pub struct SpiTransaction<'a: 'b, 'b, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    dev_ref: &'b mut SpiDevice<'a, SPI, CS>,
    cs: Option<CS>
}


impl<'a: 'b, 'b, SPI, CS> SpiDevice<'a, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    pub fn new(spi: &'a mut SPI, mut cs: CS) -> SpiResult<Self, CS> {
        cs.set_high().map_err(|x| SpiError::PinError(x))?;
        Ok(SpiDevice { spi, cs: Some(cs) })
    }

    pub async fn start_transaction(&'b mut self) -> SpiResult<SpiTransaction<'a, 'b, SPI, CS>, CS> {
        let mut cs = self.cs.take().ok_or(SpiError::AlreadyStarted)?;
        cs.set_low().map_err(|x| SpiError::PinError(x))?;
        Mono::delay(10.micros()).await;
        Ok(SpiTransaction {
            dev_ref: self,
            cs: Some(cs)
        })
    }

    fn restore(&mut self, cs: CS) {
        self.cs = Some(cs);
    }
}


impl<'a: 'b, 'b, SPI, CS> SpiErrorType for SpiTransaction<'a, 'b, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    type Error = SPI::Error;
}

impl<'a: 'b, 'b, SPI, CS> SpiBus<u8> for SpiTransaction<'a, 'b, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.dev_ref.spi.read(words)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.dev_ref.spi.write(words)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.dev_ref.spi.transfer(read, write)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.dev_ref.spi.transfer_in_place(words)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.dev_ref.spi.flush()
    }
}

impl<'a: 'b, 'b, SPI, CS> Drop for SpiTransaction<'a, 'b, SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin
{
    fn drop(&mut self) {
        let mut cs = self.cs.take().unwrap();
        cs.set_high().unwrap();
        self.dev_ref.restore(cs)
    }
}