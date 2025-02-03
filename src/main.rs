#![no_std]
#![no_main]

mod boot;
mod bsp;
mod cdc;
mod spi;
mod sx1280;

use defmt_rtt as _;
use panic_probe as _;
use portable_atomic as _;
use rtic_monotonics::rp2040::prelude::*;
rp2040_timer_monotonic!(Mono);

#[rtic::app(device = rp2040_hal::pac, peripherals = true, dispatchers = [XIP_IRQ])]
mod app {
    use super::*;

    use core::mem::MaybeUninit;
    use cortex_m::peripheral::NVIC;
    use defmt::{error, info, trace};
    
    use rp2040_hal::{
        usb::UsbBus,
        pac::Interrupt
    };
    use rtic_sync::channel::{Receiver, Sender};
    use rtic_sync::make_channel;
    use usb_device::class_prelude::*;
    use crate::bsp::{Board, types::*};
    use crate::cdc::CDCDevice;
    use crate::spi::SpiDevice;
    use crate::sx1280::commands::clear_irq::ClearIrqCommand;
    use crate::sx1280::SX1280;
    use crate::sx1280::commands::get_status::GetStatusCommand;
    use crate::sx1280::commands::set_buffer_base_address::SetBufferBaseAddressCommand;
    use crate::sx1280::commands::set_modulation_parameters::{Bandwidth, CodingRate, SetLoraModulationParameters, SpreadingFactor};
    use crate::sx1280::commands::set_packet_parameters::{LoRaCrcMode, LoRaHeaderType, LoRaIQMode, SetLoraPacketParameters};
    use crate::sx1280::commands::set_packet_type::SetPacketTypeCommand;
    use crate::sx1280::commands::set_rf_frequency::{IntoRFFrequency, SetRFFrequencyCommand};
    use crate::sx1280::commands::set_tx::{SetTxModeCommand, TxPeriod};
    use crate::sx1280::commands::set_tx_parameters::{SetTxParametersCommand, TxRampTime};
    use crate::sx1280::commands::{PeriodBase, SX1280Interrupt};
    use crate::sx1280::lora::ModeLoRa;
    use crate::sx1280::registers::rx_gain::RxGain;
    use crate::sx1280::uninitialized::ModeUninitialized;

    #[shared]
    struct Shared {
    }

    #[local]
    struct Local {
        dev: CDCDevice<'static, UsbBus>,
        uart_tx_queue: Receiver<'static, u8, 32>,
        uart_recv: Sender<'static, u8, 32>,
    }

    #[init(local = [
        usb_bus: MaybeUninit<UsbBusAllocator<UsbBus>> = MaybeUninit::uninit(),
        spi_bus: MaybeUninit<Spi1> = MaybeUninit::uninit(),
    ])]
    fn init(cx: init::Context) -> (Shared, Local) {
        info!("Starting");
        trace!("Init Board");
        let board = Board::init(cx.device).ok().unwrap();
        let usb_bus = cx.local.usb_bus.write(board.usb_bus);

        trace!("Init USB Dev");
        let dev = CDCDevice::init(
            usb_bus,
            0x16c0, 0x27dd,
            "SF", "cdctest", "1"
        ).unwrap();

        trace!("Init SPI devices");
        let spi = cx.local.spi_bus.write(board.spi_1);
        let sx_a = SpiDevice::new(spi, board.pin_cs_a).ok().unwrap();
        let sx_a_dev = SX1280::new(sx_a, board.pin_busy_a, board.pin_reset_a).ok().unwrap();

        let (uart_recv, uart_rx_queue) = make_channel!(u8, 32);
        let (uart_send, uart_tx_queue) = make_channel!(u8, 32);

        info!("Start Scheduling");
        usb_rx::spawn(uart_rx_queue, uart_send, sx_a_dev).ok().unwrap();

        (
            Shared {
            },
            Local {
                dev,
                uart_tx_queue,
                uart_recv,
            },
        )
    }

    #[task(binds=USBCTRL_IRQ, local = [dev, uart_tx_queue, uart_recv], priority=1)]
    fn on_usb(ctx: on_usb::Context) {
        let mut recv_buffer = [1u8; 32];
        for i in 0..32 {
            if let Ok(b) = ctx.local.uart_tx_queue.try_recv() {
                let _ = ctx.local.dev.write_byte(b);
            } else {
                ctx.local.dev.flush();
                break;
            }
        }
        ctx.local.dev.poll();
        for i in 0..ctx.local.dev.read(&mut recv_buffer) {
            if let Err(_) = ctx.local.uart_recv.try_send(recv_buffer[i]) {
                error!("Lost Serial byte {}", recv_buffer[i]);
            }
        }
    }

    #[task(priority=1)]
    async fn usb_rx(
        _: usb_rx::Context,
        mut rx_queue: Receiver<'static, u8, 32>,
        mut uart_tx: Sender<'static, u8, 32>,
        sx: SX1280<'static, Spi1, PinCSA, PinBusyA, PinResetA, ModeUninitialized>
    ) {

        let mut dev = sx.reset().await.ok().unwrap();
        dev.wait_for_busy(1000).await.ok().unwrap();
        let mut dev = dev.set_operating_mode::<ModeLoRa>().await.ok().unwrap();
        dev.wait_for_busy(1000).await.ok().unwrap();

        dev.command_and_wait(ClearIrqCommand(SX1280Interrupt::all())).await.ok().unwrap();
        dev.command_and_wait(SetRFFrequencyCommand(2_450_000_000)).await.ok().unwrap();
        dev.command_and_wait(SetLoraModulationParameters{
            coding_rate: CodingRate::CR4_5,
            bandwidth: Bandwidth::BW406k25Hz,
            spreading_factor: SpreadingFactor::SF8,
        }).await.ok().unwrap();
        dev.command_and_wait(SetTxParametersCommand{
            ramp: TxRampTime::Ramp20us,
            power: -18,
        }).await.ok().unwrap();
        dev.command_and_wait(SetLoraPacketParameters{
            iq_mode: LoRaIQMode::Standard,
            crc_mode: LoRaCrcMode::Enabled,
            payload_length: 128,
            header_type: LoRaHeaderType::Explicit,
            preamble_length: 8.into(),
        }).await.ok().unwrap();
        dev.command_and_wait(SetBufferBaseAddressCommand{
            rx_base_address: 0,
            tx_base_address: 128,
        }).await.ok().unwrap();
        dev.write_buffer(128, &[10, 20, 30, 40, 50, 60, 78]).await.ok().unwrap();
        dev.command_and_wait(SetTxModeCommand{
            period: TxPeriod::OneShot,
            period_base: PeriodBase::Base4ms,
        }).await.ok().unwrap();

        let status = dev.command(GetStatusCommand).await.ok().unwrap();
        let status = status.into_bits();

        while let Ok(b) = rx_queue.recv().await {
            let _ = uart_tx.send(b).await;
            let _ = uart_tx.send(((status >> 6) & 7) + 48).await;
            let _ = uart_tx.send(((status >> 3) & 7) + 48).await;
            let _ = uart_tx.send((status & 7) + 48).await;
            NVIC::pend(Interrupt::USBCTRL_IRQ);
        }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfi(); // put the MCU in sleep mode until interrupt occurs
        }
    }
}