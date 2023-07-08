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
    // function selects which peripheral is in control of the GPIO
    // Specify the target type and use `.into_mode()`
    let led_pin: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.led.into_mode();


    // PIN id for use inside of PIO
    let led_pin_id = led_pin.id().num;

    // Initialize and start PIO
    //
    // Create a new PIO wrapper and split the state machines into individual objects.
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    // source: https://docs.rs/rp2040-hal/0.8.2/rp2040_hal/pio/trait.PIOExt.html#method.split


    // 4. Write, Load, and Install PIO Program

    // write and load a pio program
    let pio_program = pio_file!("./examples/pio_blinky.pio", select_program("pio_blinky"),);
    
    // Allocates space in instruction memory and installs the program. 
    // The installed program that can be used to configure a StateMachine via PIOBuilder.
    let installed = pio0.install(&pio_program.program).unwrap();


    // 5. Configure, Build, and Initialize State Machine

    // as slow as possible (0 is interpreted as 65536)
    let (sys_div_int, sys_div_frac) = (0, 0);

    // PIOBuilder configures and deploys a PIO program into a StateMachine
    let (mut sm0, _, _) = PIOBuilder::from_program(installed)
        .set_pins(led_pin_id, 1)
        .clock_divisor_fixed_point(sys_div_int, sys_div_frac) // The clock is based on the sys_clk and will execute an instruction every (int + (frac/256)) ticks.
        .build(sm0);
        //.autopull(true)
        //.autopush(true)
        //.side_set_pin_base(led_pin_id)



    // StateMachine

    // each state machine starts off with the state machine
    let (sm0_div_int, sm0_div_frac) = (0, 0);
    sm0.clock_divisor_fixed_point(sm0_div_int, sm0_div_frac);

    // functions to explore 
    //
    // drain_tx_fifo
    // set_pindirs
    // synchronize_with // sm0.synchronize_with(sm1).and_with(sm2);
    // with 
    // restart


    // The GPIO pin needs to be configured as an output.
    sm0.set_pindirs([(led_pin_id, hal::pio::PinDir::Output)]);

    // Initialize and start PIO
    sm0.start();

    // PIO runs in background, independently from CPU, confirm by having empty loop bellow
    loop {}
}

// End of file