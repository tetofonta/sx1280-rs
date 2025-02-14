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
    use crate::sx1280::{SX1280Error, SX1280};
    use crate::sx1280::commands::get_status::GetStatusCommand;
    use crate::sx1280::commands::set_buffer_base_address::SetBufferBaseAddressCommand;
    use crate::sx1280::commands::set_modulation_parameters::{Bandwidth, CodingRate, SetLoraModulationParameters, SpreadingFactor};
    use crate::sx1280::commands::set_packet_parameters::{LoRaCrcMode, LoRaHeaderType, LoRaIQMode, SetLoraPacketParameters};
    use crate::sx1280::commands::set_packet_type::SetPacketTypeCommand;
    use crate::sx1280::commands::set_rf_frequency::{IntoRFFrequency, SetRFFrequencyCommand};
    use crate::sx1280::commands::set_tx::{SetTxModeCommand, TxPeriod};
    use crate::sx1280::commands::set_tx_parameters::{SetTxParametersCommand, TxRampTime};
    use crate::sx1280::commands::{PeriodBase, SX1280Interrupt};
    use crate::sx1280::commands::get_irq_status::GetIrqStatusCommand;
    use crate::sx1280::commands::set_irq_params::SetIRQParametersCommand;
    use crate::sx1280::commands::set_rx::{RxPeriod, SetRxModeCommand};
    use crate::sx1280::commands::set_standby::{SetStandbyModeCommand, StandbyMode};
    use crate::sx1280::lora::ModeLoRa;
    use crate::sx1280::registers::frequency_compensation_mode::FrequencyCompensationMode;
    use crate::sx1280::registers::rx_gain::{RxGain, RxGainSensitivity};
    use crate::sx1280::registers::sf_additional_configuration::SFAdditionalConfiguration;
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
        spi_bus_a: MaybeUninit<Spi0> = MaybeUninit::uninit(),
        spi_bus_b: MaybeUninit<Spi1> = MaybeUninit::uninit(),
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
        let spi_a = cx.local.spi_bus_a.write(board.spi_0);
        let sx_a = SpiDevice::new(spi_a, board.pin_cs_a).ok().unwrap();
        let sx_a_dev = SX1280::new(sx_a, board.pin_busy_a, board.pin_reset_a).ok().unwrap();

        let spi_b = cx.local.spi_bus_b.write(board.spi_1);
        let sx_b = SpiDevice::new(spi_b, board.pin_cs_b).ok().unwrap();
        let sx_b_dev = SX1280::new(sx_b, board.pin_busy_b, board.pin_reset_b).ok().unwrap();

        let (uart_recv, uart_rx_queue) = make_channel!(u8, 32);
        let (uart_send, uart_tx_queue) = make_channel!(u8, 32);

        info!("Start Scheduling");
        usb_rx::spawn(uart_rx_queue, uart_send, sx_a_dev, sx_b_dev).ok().unwrap();

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
        sx_a: SX1280<'static, Spi0, PinCSA, PinBusyA, PinResetA, ModeUninitialized>,
        sx_b: SX1280<'static, Spi1, PinCSB, PinBusyB, PinResetB, ModeUninitialized>
    ) {

        trace!("Resetting...");
        let mut tx = sx_b.reset().await.ok().unwrap();
        let mut rx = sx_a.reset().await.ok().unwrap();
        trace!("Reset complete...");


        tx.wait_for_busy(1000).await.ok().unwrap();
        rx.wait_for_busy(1000).await.ok().unwrap();
        tx.command_and_wait(SetStandbyModeCommand { mode: StandbyMode::StandbyRC }, 1000).await.ok().unwrap();
        rx.command_and_wait(SetStandbyModeCommand { mode: StandbyMode::StandbyRC }, 1000).await.ok().unwrap();

        let mut tx = tx.set_operating_mode::<ModeLoRa>().await.ok().unwrap();
        let mut rx = rx.set_operating_mode::<ModeLoRa>().await.ok().unwrap();
        tx.wait_for_busy(1000).await.ok().unwrap();
        rx.wait_for_busy(1000).await.ok().unwrap();

        info!("TX Config...");
        //tx_config
        {
            tx.command_and_wait(SetRFFrequencyCommand(2_495_000_000), 1000).await.ok().unwrap();
            tx.command_and_wait(SetBufferBaseAddressCommand { rx_base_address: 0, tx_base_address: 128 }, 1000).await.ok().unwrap();
            tx.command_and_wait(SetLoraModulationParameters {
                bandwidth: Bandwidth::BW203k125Hz,
                coding_rate: CodingRate::CR4_7,
                spreading_factor: SpreadingFactor::SF7
            }, 1000).await.ok().unwrap();
            tx.write_register(SFAdditionalConfiguration::SF7_8).await.ok().unwrap();
            tx.write_register(FrequencyCompensationMode(1)).await.ok().unwrap();
            tx.command_and_wait(SetLoraPacketParameters {
                crc_mode: LoRaCrcMode::Enabled,
                header_type: LoRaHeaderType::Explicit,
                iq_mode: LoRaIQMode::Standard,
                payload_length: 64,
                preamble_length: 20.into(),
            }, 1000).await.ok().unwrap();
            tx.command_and_wait(SetTxParametersCommand {
                power: -18,
                ramp: TxRampTime::Ramp10us
            }, 1000).await.ok().unwrap();
            tx.write_buffer(128, &[0, 1, 2, 8, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7]).await.ok().unwrap();
            tx.command_and_wait(SetIRQParametersCommand {
                dio_mask: [SX1280Interrupt::empty(); 3],
                irq_mask: SX1280Interrupt::TxDone | SX1280Interrupt::RXTXTimeout,
            }, 1000).await.ok().unwrap();
            tx.command_and_wait(ClearIrqCommand(SX1280Interrupt::all()), 1000).await.ok().unwrap();
        }

        info!("RX Config...");
        //rx_config
        {
            rx.command_and_wait(SetRFFrequencyCommand(2_495_000_000), 1000).await.ok().unwrap();
            rx.command_and_wait(SetBufferBaseAddressCommand { rx_base_address: 0, tx_base_address: 128 }, 1000).await.ok().unwrap();
            rx.command_and_wait(SetLoraModulationParameters {
                bandwidth: Bandwidth::BW203k125Hz,
                coding_rate: CodingRate::CR4_7,
                spreading_factor: SpreadingFactor::SF7
            }, 1000).await.ok().unwrap();
            rx.write_register(SFAdditionalConfiguration::SF7_8).await.ok().unwrap();
            rx.write_register(FrequencyCompensationMode(1)).await.ok().unwrap();
            rx.command_and_wait(SetLoraPacketParameters {
                crc_mode: LoRaCrcMode::Enabled,
                header_type: LoRaHeaderType::Explicit,
                iq_mode: LoRaIQMode::Standard,
                payload_length: 32,
                preamble_length: 10.into(),
            }, 1000).await.ok().unwrap();
            rx.command_and_wait(SetIRQParametersCommand {
                dio_mask: [SX1280Interrupt::empty(); 3],
                irq_mask: SX1280Interrupt::RxDone | SX1280Interrupt::RXTXTimeout | SX1280Interrupt::CRCError,
            }, 1000).await.ok().unwrap();
            rx.command_and_wait(ClearIrqCommand(SX1280Interrupt::all()), 1000).await.ok().unwrap();
            // rx.write_register(RxGain::new().with_sensitivity(RxGainSensitivity::LowSensitivity)).await.ok().unwrap();
        }

        info!("Config Complete!");

        let mut x = 0;
        loop {

            // place receiver in receive mode
            rx.command_and_wait(ClearIrqCommand(SX1280Interrupt::all()), 1000).await.ok().unwrap();
            let rx_irq = rx.command(GetIrqStatusCommand).await.ok().unwrap();
            info!("RX IRQ STATUS: {}/{}", rx_irq, SX1280Interrupt::RxDone);
            rx.command_and_wait(SetRxModeCommand{
                period_base: PeriodBase::Base4ms,
                period: RxPeriod::OneShot
            }, 10000).await.ok().unwrap();
            Mono::delay(1000.millis()).await;


            let current_time = Mono::now();
            tx.command_and_wait(SetTxModeCommand {
                period: TxPeriod::NoTimeout,
                period_base: PeriodBase::Base1ms
            }, 10000).await.ok().unwrap();


            let response = tx.wait_for_irq(SX1280Interrupt::TxDone, true, 10000).await;
            match response {
                Ok(_) => {}
                Err(SX1280Error::Timeout) => {error!("timeout")}
                Err(SX1280Error::Busy) => {error!("busy")}
                Err(_) => {error!("other")}
            }
            let duration = Mono::now().checked_duration_since(current_time).unwrap().to_millis();
            info!("{} - Transmitted (took {}ms)", x, duration);
            x += 1;

            Mono::delay(1000.millis()).await;
            let rx_irq = rx.command(GetIrqStatusCommand).await.ok().unwrap();
            info!("RX IRQ STATUS: {}/{}", rx_irq, SX1280Interrupt::RxDone);
            if rx_irq.contains(SX1280Interrupt::RxDone) {
                let mut data = [0u8; 10];
                rx.read_buffer(0, &mut data).await.ok().unwrap();
                info!("packet beginning {}", data);
            }

            Mono::delay(10000.millis()).await;
        }

        while let Ok(b) = rx_queue.recv().await {
            let _ = uart_tx.send(b).await;
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