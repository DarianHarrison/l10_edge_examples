#![no_std]
#![no_main]

// general
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

// pio
use hal::pio::{PIOBuilder};
use pio_proc::pio_file;

#[entry]
fn main() -> ! {

    /////////////////////
    // BOARD SETUP
    /////////////////////

    // 0. Grab our singleton objects
    //
    let mut pac = pac::Peripherals::take().unwrap(); // rp2040 peripherals
    //let core = pac::CorePeripherals::take().unwrap(); // cortex_m peripherals

    // 1. Configure the clocks
    //
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    //
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
    let pin_0: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio0.into_mode();
    let pin_0_id = pin_0.id().num;
    //
    // Our button input Pin on Channel B_7 Pin 1
    let pin_1 = pins.gpio1.into_pull_up_input();
    let pin_1_id = pin_1.id().num;

    // 4. Load, and Install PIO Program
    //
    // Initialize State Machines
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    //
    // Load and Install PIO Program
    let program = pio_file!("./examples/pio_button_input.pio", select_program("pio_button_input"));
    let installed = pio0.install(&program.program).unwrap();


    // 5. Configure, Build, and Initialize State Machine
    
    // as slow as possible (0 is interpreted as 65536)
    // let (sys_div_int, sys_div_frac) = (0, 0);

    // PIOBuilder configures and deploys a PIO program into a StateMachine
    let (mut sm0, _, _) = PIOBuilder::from_program(installed)
        //.set_pins(pin_0_id, 1)
        .side_set_pin_base(pin_0_id)
        .in_pin_base(pin_1_id)
        //.clock_divisor_fixed_point(sys_div_int, sys_div_frac) // The clock is based on the sys_clk and will execute an instruction every (int + (frac/256)) ticks.
        //.pull_threshold(1)
        //.autopull(true) // if you want a direct line from system (FIFO TX) to Output Pin (OUT)
        //.push_threshold(1) // specify some threshold of when to autopush
        //.autopush(true) // if you want a direct line from Input Pin (IN) to System (FIFO RX) // you can configure some threshold
        .build(sm0);

    // Set PIO PinDirs
    sm0.set_pindirs([(pin_0_id, hal::pio::PinDir::Output)]);
    sm0.set_pindirs([(pin_1_id, hal::pio::PinDir::Input)]);

    // Start state machine
    sm0.start();

    loop {}

}

// End of file