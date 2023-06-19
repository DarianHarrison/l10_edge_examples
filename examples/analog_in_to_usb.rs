//! # Pico USB Serial Example
//!
//! Creates a USB Serial device on a Pico board, with the USB driver running in
//! the main thread.
//!
//! This will create a USB Serial device echoing anything it receives. Incoming
//! ASCII characters are converted to upercase, so you can tell it is working
//! and not just local-echo!
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// Some traits we need
use embedded_hal::adc::OneShot;


// Pull in any important traits
use rp_pico::hal::prelude::*;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

// Used to demonstrate writing formatted strings
use core::fmt::Write;
use heapless::String;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then echoes any characters
/// received over USB Serial.
#[entry]
fn main() -> ! {

    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // The watchdog is a countdown timer that can restart parts of the chip if it reaches zero.
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The single-cycle I/O block controls our GPIO 
    let sio = hal::Sio::new(pac.SIO);

    // Connect PIns to Bus on this board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Set up the USB driver
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS);
    let mut counter: u32 = 0;

    // Enable ADC
    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    // Configure one of the pins as an ADC input
    let mut adc_pin_0 = pins.gpio27.into_floating_input();

    // Enable the temperature sense channel
    let mut temperature_sensor = adc.take_temp_sensor().unwrap();

    // Configure GPIO26 as an ADC input
    let mut adc_pin_0 = hal::adc::AdcPin::new(pins.gpio28);

    loop {

        // poll usb every 10 ms unless speed is configured
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // Read the raw ADC counts from the gpio sensor channel.
        adc.read(&mut adc_pin_0).unwrap();

        // put anlaog_value int 
        let mut text: String<32> = String::new();
        writeln!(&mut text, "\n\rCurrent counter: {}\r\n", analog_value).unwrap();
        writeln!(&mut text, "\n Resistor at: {} intensity",receive)
        
        // This only works reliably because the number of bytes written to
        // the serial port is smaller than the buffers available to the USB
        // peripheral. In general, the return value should be handled, so that
        // bytes not transferred yet don't get lost.
        serial.write(text.as_bytes());

    }
}

/*

let mut serial = SerialPort::new(&usb_bus);

let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    .product("Serial port")
    .device_class(USB_CLASS_CDC)
    .build();

loop {
    if !usb_dev.poll(&mut [&mut serial]) {
        continue;
    }

    // Read the raw ADC counts from the gpio sensor channel.
    let analog_value = adc.read(&mut adc_pin_0).unwrap();

    match serial.read(&mut buf[..]) {
        Ok(count) => {
            // count bytes were read to &buf[..count]
        },
        Err(UsbError::WouldBlock) => // No data received
        Err(err) => // An error occurred
    };

    match serial.write(&[0x3a, 0x29]) {
        Ok(count) => {
            // count bytes were written
        },
        Err(UsbError::WouldBlock) => // No data could be written (buffers full)
        Err(err) => // An error occurred
    };
}

*/