#![no_std]
#![no_main]

// general
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

// pio
use hal::pio::{PIOBuilder, Running, StateMachine, Tx, ValidStateMachine, SM0};
use pio::{Instruction, InstructionOperands, OutDestination};
use pio_proc::pio_file;


fn set_rise_fall_delay<T: ValidStateMachine>(
    sm: StateMachine<(hal::pac::PIO0, SM0), Running>,
    tx: &mut Tx<T>,
    period: u32,
) -> StateMachine<(hal::pac::PIO0, SM0), Running> {
    // To make sure the inserted instructions actually use our newly written value
    // We first busy loop to empty the queue. (Which typically should be the case)
    while !tx.is_empty() {}

    let mut sm = sm.stop();
    tx.write(period);
    sm.exec_instruction(Instruction {
        operands: InstructionOperands::PULL {
            if_empty: false,
            block: false,
        },
        delay: 0,
        side_set: None,
    });
    sm.exec_instruction(Instruction {
        operands: InstructionOperands::OUT {
            destination: OutDestination::ISR,
            bit_count: 32,
        },
        delay: 0,
        side_set: None,
    });
    sm.start()
}




#[entry]
fn main() -> ! {

    /////////////////////
    // BOARD SETUP
    /////////////////////
    //
    // 0. Grab our singleton objects
    //
    let mut pac = pac::Peripherals::take().unwrap(); // rp2040 peripherals

    // 1. Configure the clocks
    //
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
    //
    // The default is to generate a 125 MHz system clock
    hal::clocks::init_clocks_and_plls(
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
    let pin_0: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.led.into_mode();
    let pin_0_id = pin_0.id().num;


    // 4. Load, and Install PIO Program(s)
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let sm0_program = pio_file!("./examples/pio_blinky.pio", select_program("pio_blinky"));
    let sm0_installed = pio0.install(&sm0_program.program).unwrap();


    // 5. Configure, Build, and Initialize State Machine
    let (mut sm0, _, mut tx) = PIOBuilder::from_program(sm0_installed)
        .side_set_pin_base(pin_0_id)
        .build(sm0);

    sm0.set_pindirs([(pin_0_id, hal::pio::PinDir::Output)]);
    let sm0 = sm0.start();


    // 7. Additional PIO Logic
    //
    // Set rise/fall duration (125Mhz - 5 cycles taken to init the each loop in the PIO program)
    let rise_fall_duration: u32 = 62500000; // 125Mhz / 2 ( ~ div 2 clock)
    let clock_delay_setup: u32 = rise_fall_duration.clone() - 5;

    set_rise_fall_delay(sm0, &mut tx, clock_delay_setup);

    // 9. Processor Runtime
    loop {}
}
// End of file