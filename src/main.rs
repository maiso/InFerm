#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;
extern crate wifi_nina;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::time::{MegaHertz};

use wifi_nina::{transport::SpiTransport, Client, Wifi};

mod usb_serial;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_internal_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    // Set up the common, shared SPI bus.
    let spi = hal::spi_master_wifi(
        &mut clocks,
        MegaHertz(10),
        peripherals.SERCOM4,
        &mut peripherals.PM,
        pins.nina_sck,
        pins.nina_mosi,
        pins.nina_miso,
        &mut pins.port,
    );

    usb_serial::usb_serial::init(
        &mut peripherals.PM,
        peripherals.USB,
        &mut core.NVIC,
        &mut clocks,
        pins.usb_dm,
        pins.usb_dp,
        &mut pins.port,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);

    let wifi_busy = pins.nina_ack.into_floating_input(&mut pins.port);
    let wifi_reset = pins.nina_resetn.into_open_drain_output(&mut pins.port);
    let wifi_cs = pins.nina_cs.into_open_drain_output(&mut pins.port);
    let wifi_delay = |d: core::time::Duration| {
        delay.delay_ms(d.as_millis() as u32);
    };
    let wifi_transport =
        Wifi::new(SpiTransport::start(spi, wifi_busy, wifi_reset, wifi_cs, wifi_delay).unwrap());


    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);


    loop {
        delay.delay_ms(255u8);
        led.set_high().unwrap();
        delay.delay_ms(255u8);
        led.set_low().unwrap();
        usb_serial::usb_serial::usb_serial_log();
    }
}

