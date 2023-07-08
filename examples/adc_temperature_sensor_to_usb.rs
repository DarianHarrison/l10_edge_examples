// The Pico has a 12-bit ADC, meaning that a read operation will return a number ranging from 0 to 4095
// Therefore, the resolution of the ADC is 3.3/4096, so roughly steps of 0.8 millivolts. 
// IMPORTANT: PIO state machines can't sample the ADC directly. If you want PIO You could use a PIO to trigger a sample, or use external ADC

#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;
use embedded_hal::adc::OneShot;

use usb_device::class_prelude::UsbBusAllocator;
use usbd_serial::SerialPort;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use heapless::String;
use core::fmt::Write;


// conversion factor for onboard pico 12 bit adc
const ADC_CONVERSION_FACTOR: f32 = 0.000805664; // 3.3 / f32::from(1u16 << 12);

// See 4.9.5 from RP2040 datasheet
fn calc_temp(adc_value: f32) -> f32 {
    let vbe: f32 = adc_value * ADC_CONVERSION_FACTOR;
    27f32 - (vbe - 0.706)
}

#[entry]
fn main() -> ! {

    /////////////////////
    // BOARD SETUP
    /////////////////////

    // 0. Grab our singleton objects

    let mut pac = pac::Peripherals::take().unwrap(); // rp2040 peripherals
    // let core = pac::CorePeripherals::take().unwrap(); // cortex_m peripherals


    // 1. Configure the clocks

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB, // ADC requires a 48MHz clock (clk_adc), which could come from the USB PLL.
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();


    // 2. I/O Config

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    /////////////////////


    // 3. Enable specific Pin functions

    // Enable ADC
    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    // Enable the temperature sense channel (configures adc pins )
    // option 1 read adc input directly
    // 
    adc.enable_temp_sensor();
    //
    // (optionally)
    // Using an ADC input shared with GPIO pin 
    let mut adc_pin_0 = pins.gpio26.into_floating_input();


    // 4. Configure USB

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


    loop {
        // check for new data
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // Read the raw ADC counts from the temperature sensor channel.
        //
        // option 1
        let temp_sens_adc_counts_1: u16 = adc.read_single();
        //
        // (optionally)
        // Request that the ADC begin a conversion on the specified pin
        let temp_sens_adc_counts_2: u16 = adc.read(&mut adc_pin_0).unwrap();
        
        // convert input counts into temperature
        let temp_1: f32 = calc_temp(temp_sens_adc_counts_1 as f32);
        let temp_2: f32 = calc_temp(temp_sens_adc_counts_2 as f32);

        // string buffer de 32 bytes
        let mut text: String<32> = String::new();
        let _ = writeln!(&mut text, "Temp 1: {:.2} °C, Temp 2: {:.2} °C", temp_1, temp_2);

        // This only works reliably because the number of bytes written to
        // the serial port is smaller than the buffers available to the USB
        // peripheral. In general, the return value should be handled, so that
        // bytes not transferred yet don't get lost.
        let _ = serial.write(text.as_bytes());
    }
}

// End of file