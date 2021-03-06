#![no_std]
#![no_main]

extern crate arduino_nano33iot as hal;

use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::entry;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;

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

    usb_serial::usb_serial::init(
        &mut peripherals.PM,
        peripherals.USB,
        &mut core.NVIC,
        &mut clocks,
        pins.usb_dm,
        pins.usb_dp,
        &mut pins.port,
    );

    let mut led = pins.led_sck.into_open_drain_output(&mut pins.port);

    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        delay.delay_ms(255u8);
        led.set_high().unwrap();
        delay.delay_ms(255u8);
        led.set_low().unwrap();
        usb_serial::usb_serial::usb_serial_log();
    }
}
