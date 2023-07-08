#![no_std]
#![no_main]

// general
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {

    /////////////////////
    // BOARD SETUP
    /////////////////////

    // 0. Grab our singleton objects
    //
    let mut pac = pac::Peripherals::take().unwrap(); // rp2040 peripherals
    let core = pac::CorePeripherals::take().unwrap(); // cortex_m peripherals

    // 1. Configure the clocks
    //
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    //
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

    // 2. I/O Config
    //
    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);
    // /
    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    /////////////////////

    // 3. Enable specific Pin functions
    //
    // Our LED output Channel A_0 Pin 0
    // 
    let mut pin_0 = pins.gpio0.into_push_pull_output();
    //
    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    loop {

        pin_0.set_high().unwrap();
        delay.delay_ms(500);
        pin_0.set_low().unwrap();
        delay.delay_ms(500);
    }

}

// End of file