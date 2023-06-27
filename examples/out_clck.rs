#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;
use pio_proc::pio_file;
use crate::hal::pio::PIOBuilder;

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

<<<<<<< HEAD
    let led_pin: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio17.into_mode();
=======
    /////////////////////


    // 3. Enable specific Pin functions

    // configure LED pin for Pio0.
    // function selects which peripheral is in control of the GPIO
    // Specify the target type and use `.into_mode()`
    let led_pin: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.led.into_mode();


    // PIN id for use inside of PIO
>>>>>>> b6a8a54614b6c0e1dbfb837da999764410a195b8
    let led_pin_id = led_pin.id().num;

    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let pio_program = pio_file!("./examples/pio_programs.pio", select_program("out_clck"),);
    let installed = pio0.install(&pio_program.program).unwrap();
    let (mut sm0, _, _) = PIOBuilder::from_program(installed)
        .set_pins(led_pin_id, 1)
        .build(sm0);

<<<<<<< HEAD
=======


    // StateMachine

    // each state machine starts off with the state machine
>>>>>>> b6a8a54614b6c0e1dbfb837da999764410a195b8
    let (sm0_div_int, sm0_div_frac) = (0, 0);
    sm0.clock_divisor_fixed_point(sm0_div_int, sm0_div_frac);
    sm0.set_pindirs([(led_pin_id, hal::pio::PinDir::Output)]);
    sm0.start();

    loop {}
}
