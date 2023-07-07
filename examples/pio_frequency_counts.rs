// Note: The maximum frequency on this frequency is 250MHZ, which is the 2N2222 transistor's transition frequency.

#![no_std]
#![no_main]

// general
use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

// pio
use hal::pio::{PIOBuilder, Running, StateMachine, Tx, Rx, ValidStateMachine, SM0, SM1};
use pio::{Instruction, InstructionOperands, OutDestination};
use pio_proc::pio_file;

// usb
use usb_device::class_prelude::UsbBusAllocator;
use usbd_serial::SerialPort;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use heapless::String;
use core::fmt::Write;

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
    //
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
    // Our PWM output Channel A_0 Pin 0
    // 
    let pin_0: hal::gpio::Pin<_, hal::gpio::FunctionPio0> = pins.gpio0.into_mode();
    let pin_0_id = pin_0.id().num;
    //
    // Our Transistor pull-up Input Pin on Channel B_7 Pin 1
    let pin_1 = pins.gpio1.into_pull_up_input();
    let pin_1_id = pin_1.id().num;


    // 4. Load, and Install PIO Program(s)
    //
    // Initialize State Machines
    let (mut pio0, sm0, sm1, _, _) = pac.PIO0.split(&mut pac.RESETS);
    //
    // Load and Install PIO Program
    let sm0_program = pio_file!("./examples/pio_frequency_counts.pio", select_program("pwm_generator"));
    let sm1_program = pio_file!("./examples/pio_frequency_counts.pio", select_program("pio_frequency_counts"));

    let sm0_installed = pio0.install(&sm0_program.program).unwrap();
    let sm1_installed = pio0.install(&sm1_program.program).unwrap();

    // 5. Configure, Build, and Initialize State Machine
    // PIOBuilder configures and deploys a PIO program into a StateMachine
    let (mut sm0, _, mut tx) = PIOBuilder::from_program(sm0_installed)
        .set_pins(pin_0_id, 1)
        .side_set_pin_base(pin_0_id)
        .build(sm0);

    let (mut sm1, mut rx, _) = PIOBuilder::from_program(sm1_installed)
        .in_pin_base(pin_1_id)
        .build(sm1);

    // set PIO PinDirs for StateMachines
    sm0.set_pindirs([(pin_0_id, hal::pio::PinDir::Output)]); // Theis GPIO pin are configured as an output.
    sm1.set_pindirs([(pin_1_id, hal::pio::PinDir::Input)]); // Theis GPIO pin are configured as an output.


    // sync both state machines
    // sm0.synchronize_with(&mut sm1);


    // Start state machine
    let sm0 = sm0.start();
    sm1.start();


    // 6. Configure USB

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
        .manufacturer("Lion10")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();


    // 7. Additional PIO Logic
    //
    // Set period
    pio_pwm_set_period(sm0, &mut tx, u16::MAX as u32 - 1);


    // 8. Processor Logic
    //
    // The delay object lets us wait for specified amounts of time (in milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    //
    // Loop forever and adjust duty cycle to make te led brighter
    let mut level = 0;

    // 9. Processor Runtime
    loop {

        info!("Level = {}", level);
        pio_pwm_set_level(&mut tx, level * level);
        level = (level + 1) % 256;
        delay.delay_ms(10);

        // check for new data
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        // Get the next element from RX FIFO. Returns None if the FIFO is empty.
        let rx_value: u32 = rx.read().unwrap_or(3); // 3 = something not right, input must be 0 or 1

        // string buffer de 32 bytes
        let mut text: String<32> = String::new();
        let _ = writeln!(&mut text, "Digital Input Value: {rx_value}");

        // This only works reliably because the number of bytes written to
        // the serial port is smaller than the buffers available to the USB
        // peripheral. In general, the return value should be handled, so that
        // bytes not transferred yet don't get lost.
        let _ = serial.write(text.as_bytes());
    }
}
// End of file