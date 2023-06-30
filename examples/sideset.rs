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
    let _clocks = hal::clocks::init_clocks_and_plls(
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

    // configure LED pin for Pio0.
    let led_pin_15: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio15.into_mode();
    let led_pin_15_id = led_pin_15.id().num;

    let led_pin_14: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio14.into_mode();
    let led_pin_14_id = led_pin_14.id().num;

    // Initialize and start PIO
    //
    // Create a new PIO wrapper and split the state machines into individual objects.
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    // source: https://docs.rs/rp2040-hal/0.8.2/rp2040_hal/pio/trait.PIOExt.html#method.split


    // 4. Write, Load, and Install PIO Program

    // write and load a pio program
    let pio_program = pio_file!("./examples/sideset.pio", select_program("sideset"),);
    
    // Allocates space in instruction memory and installs the program. 
    // The installed program that can be used to configure a StateMachine via PIOBuilder.
    let installed = pio0.install(&pio_program.program).unwrap();


    // 5. Configure, Build, and Initialize State Machine

    // PIOBuilder configures and deploys a PIO program into a StateMachine
    let (mut sm0, _, _) = PIOBuilder::from_program(installed)
        .set_pins(led_pin_15_id, 1)
        .side_set_pin_base(led_pin_14_id)
        //.autopull(true)
        //.autopush(true)
        .build(sm0);

    // StateMachine
    let (sm0_div_int, sm0_div_frac) = (0, 0);
    sm0.clock_divisor_fixed_point(sm0_div_int, sm0_div_frac); // as slow as possible (0 is interpreted as 65536)
    sm0.set_pindirs([(led_pin_15_id, hal::pio::PinDir::Output)]); // Theis GPIO pin are configured as an output.
    sm0.set_pindirs([(led_pin_14_id, hal::pio::PinDir::Output)]); // Theis GPIO pin are configured as an output.
    sm0.start(); // Initialize and start PIO

    // functions to explore 
    //
    // drain_tx_fifo
    // synchronize_with // sm0.synchronize_with(sm1).and_with(sm2);
    // with 
    // restart

    // PIO runs in background, independently from CPU, confirm by having empty loop bellow
    loop {}
}

// End of file