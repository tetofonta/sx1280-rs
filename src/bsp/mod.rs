pub mod types;

use embedded_hal::spi::{Mode, MODE_0};
use rp2040_hal::pac::{Peripherals, SIO, USBCTRL_REGS, USBCTRL_DPRAM, RESETS, IO_BANK0, PADS_BANK0};
use rp2040_hal::clocks::{init_clocks_and_plls, InitError, UsbClock};
use rp2040_hal::{Clock, Sio, Spi, Watchdog};
use rp2040_hal::fugit::{HertzU32, RateExtU32};
use rp2040_hal::usb::UsbBus;
use rp2040_hal::gpio::{Pins, FunctionSpi, PinState};
use rp2040_hal::spi::{Enabled, SpiDevice, ValidSpiPinout};
use usb_device::bus::UsbBusAllocator;
use crate::Mono;
use crate::bsp::types::*;

pub const XTAL_FREQ_HZ: u32 = 12_000_000u32;


pub struct Board {
    pub usb_bus: UsbBusAllocator<UsbBus>,
    pub spi_1: Spi1,
    pub pin_reset_a: PinResetA,
    pub pin_busy_a: PinBusyA,
    pub pin_cs_a: PinCSA,
}

impl Board {
    pub fn init(peripherals: Peripherals) -> Result<Self, InitError> {
        let mut resets = peripherals.RESETS;
        Mono::start(peripherals.TIMER, &resets);
        let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
        let clocks = init_clocks_and_plls(
            XTAL_FREQ_HZ,
            peripherals.XOSC,
            peripherals.CLOCKS,
            peripherals.PLL_SYS,
            peripherals.PLL_USB,
            &mut resets,
            &mut watchdog,
        )?;

        let (spi1_pin, pin_reset_a, pin_busy_a, pin_cs_a) = Self::init_gpio(peripherals.SIO, peripherals.IO_BANK0, peripherals.PADS_BANK0, &mut resets);

        Ok(Self {
            usb_bus: Self::init_usb(peripherals.USBCTRL_REGS, peripherals.USBCTRL_DPRAM, clocks.usb_clock, &mut resets),
            spi_1: Self::init_spi(peripherals.SPI1, spi1_pin.to_pins(), &mut resets, clocks.peripheral_clock.freq(), 1.MHz(), MODE_0),
            pin_reset_a,
            pin_busy_a,
            pin_cs_a,
        })
    }

    fn init_gpio(s: SIO, io_bank_0: IO_BANK0, pands_bank_0: PADS_BANK0, r: &mut RESETS) -> (Spi1Pins, PinResetA, PinBusyA, PinCSA) {
        let sio = Sio::new(s);
        let pins = Pins::new(
            io_bank_0,
            pands_bank_0,
            sio.gpio_bank0,
            r,
        );

        let _ = pins.gpio14.into_push_pull_output_in_state(PinState::High);

        (
            Spi1Pins {
                mosi: pins.gpio11.into_function::<FunctionSpi>(),
                miso: pins.gpio12.into_function::<FunctionSpi>(),
                sck: pins.gpio10.into_function::<FunctionSpi>()
            },
            pins.gpio2.into_push_pull_output_in_state(PinState::High),
            pins.gpio3.into_pull_up_input(),
            pins.gpio5.into_push_pull_output_in_state(PinState::High),
        )
    }

    fn init_spi<D: SpiDevice, P: ValidSpiPinout<D>>(spi: D, pins: P, resets: &mut RESETS, clock: HertzU32, spi_clock: HertzU32, mode: Mode)  -> Spi<Enabled, D, P> {
        let spi_bus = Spi::<_, _, _, 8>::new(spi, pins);
        spi_bus.init(
            resets,
            clock,
            spi_clock,
            mode,
        )
    }

    fn init_usb(ctrl: USBCTRL_REGS, dpram: USBCTRL_DPRAM, clk: UsbClock, r: &mut RESETS) -> UsbBusAllocator<UsbBus> {
        UsbBusAllocator::new(UsbBus::new(
            ctrl,
            dpram,
            clk,
            true,
            r,
        ))
    }

}