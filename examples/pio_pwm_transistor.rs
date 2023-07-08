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


// logging
use defmt::info;
use defmt_rtt as _;


/// Set pio pwm period
///
/// This uses a sneaky trick to set a second value besides the duty cycle.
/// We first write a value to the tx fifo. But instead of the normal instructions we
/// have stopped the state machine and inject our own instructions that move the written value to the ISR.
fn pio_pwm_set_period<T: ValidStateMachine>(
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

/// Set pio pwm duty cycle
///
/// The value written to the TX FIFO is used directly by the normal pio program
fn pio_pwm_set_level<T: ValidStateMachine>(tx: &mut Tx<T>, level: u32) {
    // Write duty cycle to TX Fifo
    tx.write(level);
}


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
    let mut pin_0: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio0.into_mode();
    let mut pin_0_id = pin_0.id().num;
    //
    // Our button input Pin on Channel B_7 Pin 1
    // let pin_1 = pins.gpio1.into_pull_up_input();
    // let pin_1_id = pin_1.id().num;

    // 4. Load, and Install PIO Program
    //
    // Initialize State Machines
    let (mut pio0, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    //
    // Load and Install PIO Program
    let program = pio_file!("./examples/pio_pwm_transistor.pio", select_program("pio_pwm_transistor"));
    let installed = pio0.install(&program.program).unwrap();


    // 5. Configure, Build, and Initialize State Machine
    
    // as slow as possible (0 is interpreted as 65536)
    // let (sys_div_int, sys_div_frac) = (0, 0);

    // PIOBuilder configures and deploys a PIO program into a StateMachine
    let (mut sm0, _, mut tx) = PIOBuilder::from_program(installed)
        // .in_pin_base(pin_1_id)
        // .build(sm0);
        .set_pins(pin_0_id, 1)
        .side_set_pin_base(pin_0_id)
        .build(sm0);


    // Set PIO PinDirs
    sm0.set_pindirs([(pin_0_id, hal::pio::PinDir::Output)]);

    // Start state machine
    let sm0 =  sm0.start();



    // Set period
    pio_pwm_set_period(sm0, &mut tx, u16::MAX as u32 - 1);


    // 8. Configure, Build, and Initialize State Machine

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());


    // Loop forever and adjust duty cycle to make te led brighter
    let mut level = 0;
    loop {
        info!("Level = {}", level);
        pio_pwm_set_level(&mut tx, level * level);
        level = (level + 1) % 256;
        delay.delay_ms(10);
    }

}

// End of file