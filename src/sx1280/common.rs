use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use rp2040_hal::fugit::ExtU64;
use core::marker::PhantomData;
use rtic_monotonics::Monotonic;
use crate::Mono;
use crate::sx1280::{SX1280Error, SX1280Mode, SX1280ModeValid, SX1280Result, SX1280};
use crate::sx1280::commands::set_packet_type::SetPacketTypeCommand;
use crate::sx1280::commands::SX1280Command;
use crate::sx1280::registers::SX1280Register;
use crate::sx1280::uninitialized::ModeUninitialized;



impl<'a, SPI: SpiBus<u8>, CS: OutputPin, BUSY: InputPin, RESET: OutputPin, MODE: SX1280Mode> SX1280<'a, SPI, CS, BUSY, RESET, MODE> {

    pub async fn reset(mut self) -> SX1280Result<SX1280<'a, SPI, CS, BUSY, RESET, ModeUninitialized>, Self> {
        self.reset_pin.set_low()?;
        Mono::delay(100.millis()).await;
        self.reset_pin.set_high()?;
        Ok(SX1280{
            reset_pin: self.reset_pin,
            busy_pin: self.busy_pin,
            spi: self.spi,
            _phantom: PhantomData { }
        })
    }

    pub fn ensure_not_busy(&mut self) -> SX1280Result<(), Self> {
        if self.busy_pin.is_high()? {
            return Err(SX1280Error::Busy)
        }
        Ok(())
    }

    pub async fn write_register<T: SX1280Register<MODE>>(&mut self, reg: T) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let bytes = reg.as_write_bytes();
        let mut trx = self.spi.start_transaction().await?;
        trx.write(&[0x18u8]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&T::ADDRESS.to_be_bytes()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(bytes.as_ref()).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(())
    }

    pub async fn read_register<T: SX1280Register<MODE>>(&mut self) -> SX1280Result<T, Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trx.write(&[0x19u8]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&T::ADDRESS.to_be_bytes()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&[0u8]).map_err(|x| SX1280Error::SpiError(x))?;
        let mut buffer: T::BufferType = T::BufferType::default();
        trx.read(buffer.as_mut()).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(buffer.try_into()?)
    }

    pub async fn write_buffer(&mut self, offset: u8, data: &[u8]) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trx.write(&[0x1Au8, offset]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(data).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(())
    }

    pub async fn read_buffer(&mut self, offset: u8, data: &mut [u8]) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trx.write(&[0x1Bu8, offset, 0]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.read(data).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(())
    }

    pub async fn command<T: SX1280Command<MODE>>(&mut self, command: T) -> SX1280Result<T::ResponseType, Self> {
        self.ensure_not_busy()?;
        let bytes = command.as_write_bytes()?;
        let mut ret = T::ResponseBufferType::default();
        let mut opcode = [T::OPCODE];

        let mut trx = self.spi.start_transaction().await?;
        trx.transfer_in_place(&mut opcode).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(bytes.as_ref()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.transfer_in_place(ret.as_mut()).map_err(|x| SX1280Error::SpiError(x))?;

        Ok((opcode[0], ret).try_into()?)
    }

    pub async fn set_operating_mode<T: SX1280ModeValid>(mut self) -> SX1280Result<SX1280<'a, SPI, CS, BUSY, RESET, T>, Self>{
        let _ = self.command(SetPacketTypeCommand(T::PACKET_CONST)).await?;
        Ok(SX1280 {
            spi: self.spi,
            busy_pin: self.busy_pin,
            reset_pin: self.reset_pin,
            _phantom: PhantomData::<T> { },
        })
    }


}