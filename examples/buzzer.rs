//! # Pico PWM Melody Example
//!
//! Fades the LED on a Pico board using the PWM peripheral and plays a melody on a buzzer.
//!
//! This will fade in/out the LED attached to GP25 (on-board LED) and play a melody on the
//! buzzer attached to a GPIO pin (e.g., GP26).
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// Pull in any important traits
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// The minimum PWM value (i.e. LED brightness) we want
const LOW: u16 = 0;

// The maximum PWM value (i.e. LED brightness) we want
const HIGH: u16 = 25000;

// Melody notes and corresponding durations
const MELODY_NOTES: [(u32, u32); 8] = [
    (523, 200),  // C
    (587, 200),  // D
    (659, 200),  // E
    (698, 200),  // F
    (784, 400),  // G
    (880, 400),  // A
    (987, 400),  // B
    (1046, 400), // C
];

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then fades the LED and plays
/// a melody on the buzzer in an infinite loop.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
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

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // Init PWMs
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM5
    let pwm = &mut pwm_slices.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    // Our LED output
    let led_pin = pins.gpio15.into_push_pull_output();

    // Output channel B on PWM7 to the LED pin
    let channel = &mut pwm.channel_b;
    channel.output_to(led_pin);

    // Buzzer pin (e.g., GP26)
    let mut buzzer_pin = pins.gpio16.into_push_pull_output();

    // Infinite loop, fading LED up and down and playing a melody
    loop {
        // Fade LED up and down
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(8);
            channel.set_duty(i);
        }

        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(8);
            channel.set_duty(i);
        }

        // Play melody
        for (frequency, duration) in &MELODY_NOTES {
            buzzer_pin.set_high().unwrap();
            delay.delay_us(*duration);
            buzzer_pin.set_low().unwrap();
            delay.delay_ms(100);
        }
    }
}
