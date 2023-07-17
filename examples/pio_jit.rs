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
use pio_proc::pio_asm;

#[entry]
fn main() -> ! {

    /////////////////////
    // BOARD SETUP
    /////////////////////
    //
    // 0. Grab our singleton objects
    //
    let mut pac = pac::Peripherals::take().unwrap(); // rp2040 peripherals
    let core = pac::CorePeripherals::take().unwrap();

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
    //
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
    let pin_0: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.led.into_mode(); // ccd_clck
    let pin_0_id = pin_0.id().num;

    // 4. Define Pio Block and StateMachines to use
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
 
    /////////////////////
    // JIT PIO Program
    /////////////////////
    // 5. PIO Program
    //
    // a) PIO program configuration
    let mut ss: pio::SideSet = pio::SideSet::new(true,1,false);
    let mut assembler = pio::Assembler::<32>::new_with_side_set(ss);
    //
    // b) PIO program configuration
    assembler.set_with_side_set(pio::SetDestination::PINDIRS, 1, 1);
    assembler.nop_with_side_set(1);
    assembler.nop_with_side_set(0);
    //
    // c) JIT compile the PIO program
    //
    let sm0_program = assembler.assemble_program();
    let sm0_installed = pio0.install(&sm0_program).unwrap();
    //
    // d) Configure, Build, and Initialize State Machine
    let (mut sm0, _, _) = PIOBuilder::from_program(sm0_installed)
        .side_set_pin_base(pin_0_id)
        .build(sm0);
    sm0.set_pindirs([(pin_0_id, hal::pio::PinDir::Output)]);
    //
    /////////////////////

    // 6. StateMachine

    let _ = sm0.start();

    // used only for externa communication
    loop {}
}
// End of file
