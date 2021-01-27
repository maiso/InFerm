pub mod usb_serial {

    extern crate arduino_nano33iot as hal;

    use hal::clock::GenericClockController;
    // use hal::pac::{interrupt, CorePeripherals, Peripherals};
    use hal::gpio::{Floating, Input, Port};
    use hal::pac::{interrupt, PM, USB };

    use hal::usb::UsbBus;
    use usb_device::bus::UsbBusAllocator;
    use usb_device::prelude::*;
    use usbd_serial::{SerialPort, USB_CLASS_CDC};

    use cortex_m::peripheral::NVIC;

    /// Initializes the `USBSerial` singleton.
    ///
    /// # Arguments
    ///  * pm_perph: The power management peripheral
    ///  * usb_perph: The USB peripheral
    ///  * core: The `CorePeripheral` instance for NVIC modifications
    ///  * clocks: The clocks instance for USB peripheral clocking
    ///  * dm: The d- GPIO pad
    ///  * dp: The d+ GPIO pad
    ///  * port: the GPIO port
    pub fn init(
        pm_perph: &mut PM,
        usb_perph: USB,
        nvic: &mut hal::pac::NVIC,
        clocks: &mut GenericClockController,
        dm: hal::gpio::Pa24<Input<Floating>>,
        dp: hal::gpio::Pa25<Input<Floating>>,
        port: &mut Port,
    ) {
        let bus_allocator = unsafe {
            USB_ALLOCATOR = Some(hal::usb_allocator(
                usb_perph, clocks, pm_perph, dm, dp, port,
            ));
            USB_ALLOCATOR.as_ref().unwrap()
        };

        unsafe {
            USB_SERIAL = Some(SerialPort::new(&bus_allocator));
            USB_BUS = Some(
                UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x2222, 0x3333))
                    .manufacturer("Fake company")
                    .product("Serial port")
                    .serial_number("TEST")
                    .device_class(USB_CLASS_CDC)
                    .build(),
            );
        }

        unsafe {
            nvic.set_priority(interrupt::USB, 1);
            NVIC::unmask(interrupt::USB);
        }
    }

    pub fn usb_serial_log() {
        // Turn off interrupts so we don't fight with the interrupt
        cortex_m::interrupt::free(|_| unsafe {
            USB_BUS.as_mut().map(|_| {
                USB_SERIAL.as_mut().map(|serial| {
                    // Skip errors so we can continue the program
                    let _ = serial.write("Hallo Jeroen\r\n".as_bytes());
                });
            })
        });
    }

    static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
    static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
    static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

    fn poll_usb() {
        unsafe {
            USB_BUS.as_mut().map(|usb_dev| {
                USB_SERIAL.as_mut().map(|serial| {
                    usb_dev.poll(&mut [serial]);

                    // Make the other side happy
                    let mut buf = [0u8; 16];
                    let _ = serial.read(&mut buf);
                });
            });
        };
    }

    #[interrupt]
    fn USB() {
        poll_usb();
    }

}  

