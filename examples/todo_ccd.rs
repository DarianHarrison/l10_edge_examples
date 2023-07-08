// Asegúrate de tener las siguientes dependencias en tu Cargo.toml:
// [dependencies]
// rp-pico = "0.5.0"
// panic-halt = "0.2.0"
// usb-device = "0.3.0"
// usbd-serial = "0.5.0"
// heapless = "0.7.0"
// cortex-m = "0.7.0"
// cortex-m-rt = "0.7.0"

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

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
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

    let sio = hal::Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_0 = pins.gpio27.into_floating_input();
    let mut adc_pin_1 = pins.gpio26.into_floating_input();

    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    let mut serial = SerialPort::new(&usb_bus);
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2)
        .build();

    const CCD_SIZE: usize = 500; // Tamaño del arreglo de datos del CCD

    let mut ccd_data: [u16; CCD_SIZE] = [0; CCD_SIZE];     

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }
    
        // Leer los datos del CCD en el arreglo
        for i in 0..CCD_SIZE {
            ccd_data[i] = adc.read(&mut adc_pin_0).unwrap();
            // Realiza cualquier procesamiento adicional necesario aquí
        }

        for i in 0..CCD_SIZE {
            let mut text: String<32> = String::new();
            let _ = writeln!(&mut text, "CCD Data {}: {}", i, ccd_data[i]);
            let _ = serial.write(text.as_bytes());
        }

    }
}
