#![no_std]
#![no_main]

use rp_pico::entry;
use panic_halt as _;
use rp_pico::hal::pac;
use rp_pico::hal;
use embedded_hal::adc::OneShot;
// use embedded_hal::Pwm;
use rp_pico::hal::prelude::*;
use embedded_hal::PwmPin;

const LOW: u16 = 0;
const HIGH: u16 = 25000;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
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

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);
    let mut adc_pin_0 = pins.gpio27.into_floating_input();

    let pwm = &mut pwm_slices.pwm7;
    pwm.set_ph_correct();
    pwm.enable();

    let led_pin = pins.gpio15.into_push_pull_output();
    // let mut potentiometer_pin = pins.gpio28.into_analog();

    let channel_out = &mut pwm.channel_b;
    channel_out.output_to(led_pin);
    // let channel_in = &mut pwm.channel_b;
    // let mut potentiometer_pin = channel_in.input_from(pins.gpio27);

    // led_pin.set_pwm_frequency(1000.0);

    loop {
        // let value = potentiometer_pin.read().unwrap();
        // led_pin.set_duty(value);
        // delay.delay_ms(100);
        let value: u16 = adc.read(&mut adc_pin_0).unwrap();
        channel_out.set_duty(value);
        
        // for i in (LOW..=HIGH).skip(100) {
        //     delay.delay_us(8);
        //     channel.set_duty(i);
        // }

        // for i in (LOW..=HIGH).rev().skip(100) {
        //     delay.delay_us(8);
        //     channel.set_duty(i);
        // }
    }
}