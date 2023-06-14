# l10-ccd



## reference

![Alt Text](./docs/datasheet.png)


* timers
* pwm
* adc
* dma
* usb
* dma to usb
* usb to host


1. 
```bash
cargo run --release --example adc_usb
```

## sources

pico board crate
https://crates.io/crates/rp-pico

Raspberry Silicon RP2040 microcontroller
https://crates.io/crates/rp2040-hal
note: this is re-exported by the board crate, so it does not need to be in cargo.toml if you implement the board rp-pico board crate 

embedded-hal
https://crates.io/crates/embedded-hal


usb host system

https://github.com/rust-embedded-community/usbd-serial

https://github.com/a1ien/rusb
https://github.com/rust-embedded-community/usb-device
