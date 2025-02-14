use core::future::poll_fn;
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal::spi::SpiBus;
use rp2040_hal::fugit::{Duration, ExtU64};
use core::marker::PhantomData;
use core::task::Poll;
use defmt::trace;
use rtic_monotonics::{Monotonic, TimeoutError};
use crate::Mono;
use crate::sx1280::{SX1280Error, SX1280Mode, SX1280ModeValid, SX1280Result, SX1280};
use crate::sx1280::commands::set_packet_type::SetPacketTypeCommand;
use crate::sx1280::commands::{SX1280Command, SX1280Interrupt};
use crate::sx1280::commands::clear_irq::ClearIrqCommand;
use crate::sx1280::commands::get_irq_status::GetIrqStatusCommand;
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

    async fn __internal_wait_for_busy(&mut self) -> SX1280Result<(), Self>{
        Mono::delay(10.millis()).await; // todo: wtf
        while self.busy_pin.is_high()? {
            Mono::delay(1.millis()).await; // todo: wtf
        }
        Ok(())
    }

    pub async fn wait_for_busy(&mut self, timeout: u64) -> SX1280Result<(), Self> {
        if self.ensure_not_busy().is_ok() { return Ok(()); }
        if timeout == 0 {return self.__internal_wait_for_busy().await}
        match Mono::timeout_after(timeout.millis(), self.__internal_wait_for_busy()).await{
            Ok(r) => r,
            Err(_) => Err(SX1280Error::Timeout)
        }
    }

    pub async fn write_register<T: SX1280Register<MODE>>(&mut self, reg: T) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let bytes = reg.as_write_bytes();
        let mut trx = self.spi.start_transaction().await?;
        trace!("WRITE REG -> [0x18] {:?} {:?}", &T::ADDRESS.to_be_bytes(), &bytes.as_ref());

        trx.write(&[0x18u8]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&T::ADDRESS.to_be_bytes()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(bytes.as_ref()).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(())
    }

    pub async fn read_register<T: SX1280Register<MODE>>(&mut self) -> SX1280Result<T, Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trace!("READ REG -> [0x19] {:?}", &T::ADDRESS.to_be_bytes());

        trx.write(&[0x19u8]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&T::ADDRESS.to_be_bytes()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(&[0u8]).map_err(|x| SX1280Error::SpiError(x))?;
        let mut buffer: T::BufferType = T::BufferType::default();
        trx.read(buffer.as_mut()).map_err(|x| SX1280Error::SpiError(x))?;
        trace!("READ REG <- {:?}", &buffer.as_ref());

        Ok(buffer.try_into()?)
    }

    pub async fn write_buffer(&mut self, offset: u8, data: &[u8]) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trace!("WRITE BUF -> [0x1A, {}] {:?}", offset, data);

        trx.write(&[0x1Au8, offset]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(data).map_err(|x| SX1280Error::SpiError(x))?;
        Ok(())
    }

    pub async fn read_buffer(&mut self, offset: u8, data: &mut [u8]) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        let mut trx = self.spi.start_transaction().await?;
        trace!("READ BUF -> [0x1B, {}, 0]", offset);

        trx.write(&[0x1Bu8, offset, 0]).map_err(|x| SX1280Error::SpiError(x))?;
        trx.read(data).map_err(|x| SX1280Error::SpiError(x))?;
        trace!("READ BUF <- {:?}", data);

        Ok(())
    }

    pub async fn command<T: SX1280Command<MODE>>(&mut self, command: T) -> SX1280Result<T::ResponseType, Self> {
        self.ensure_not_busy()?;
        let bytes = command.as_write_bytes()?;
        let mut ret = T::ResponseBufferType::default();
        let mut opcode = [T::OPCODE];

        let mut trx = self.spi.start_transaction().await?;
        trace!("COMMAND -> {:?} {:?}", &opcode, &bytes.as_ref());
        trx.transfer_in_place(&mut opcode).map_err(|x| SX1280Error::SpiError(x))?;
        trx.write(bytes.as_ref()).map_err(|x| SX1280Error::SpiError(x))?;
        trx.transfer_in_place(ret.as_mut()).map_err(|x| SX1280Error::SpiError(x))?;
        trace!("COMMAND <- {:?}", &ret.as_ref());
        Ok((opcode[0], ret).try_into()?)
    }

    pub async fn command_and_wait<T: SX1280Command<MODE>>(&mut self, command: T, timeout: u64) -> SX1280Result<T::ResponseType, Self> {
        let ret = self.command(command).await?;
        self.wait_for_busy(timeout).await?;
        Ok(ret)
    }

    pub async fn set_operating_mode<T: SX1280ModeValid>(mut self) -> SX1280Result<SX1280<'a, SPI, CS, BUSY, RESET, T>, Self>{
        let _ = self.command(SetPacketTypeCommand(T::PACKET_CONST)).await?;
        self.__internal_wait_for_busy().await?;
        Ok(SX1280 {
            spi: self.spi,
            busy_pin: self.busy_pin,
            reset_pin: self.reset_pin,
            _phantom: PhantomData::<T> { },
        })
    }
}

impl<'a, SPI: SpiBus<u8>, CS: OutputPin, BUSY: InputPin, RESET: OutputPin, MODE: SX1280ModeValid> SX1280<'a, SPI, CS, BUSY, RESET, MODE> {

    async fn __internal_wait_for_irq(&mut self, irq: SX1280Interrupt, clear: bool) -> SX1280Result<(), Self>{
        loop {
            let irqs = self.command(GetIrqStatusCommand).await?;
            if irqs.contains(irq) {
                if clear {
                    self.__internal_wait_for_busy().await?;
                    self.command(ClearIrqCommand(irq)).await?;
                    self.__internal_wait_for_busy().await?;
                }
                return Ok(())
            }
            self.__internal_wait_for_busy().await?;
        }
    }

    pub async fn wait_for_irq(&mut self, irq: SX1280Interrupt, clear: bool, timeout: u64) -> SX1280Result<(), Self> {
        self.ensure_not_busy()?;
        match Mono::timeout_after(timeout.millis(), self.__internal_wait_for_irq(irq, clear)).await{
            Ok(r) => r,
            Err(_) => Err(SX1280Error::Timeout)
        }
    }
}
