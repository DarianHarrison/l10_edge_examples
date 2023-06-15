

# l10_detector




## reference

Edge
- [] 1. configure different clock frequencies ((5) different frequencies between min and max)
- [] 2. use pwm to blink a led (many times per second)
- [] 3. use adc to capture photoresist (every 1 second)
- [] 4. send photoresist data to usb continous (every 1 second)


Core:
- [] 1. poll usb device in pin
- [] 2. deserialize & print (every 1 second)


TODOS:
- [] dma
- [] dma to usb


TODOS:
```sh
#cargo run --release --example timers
```





![Alt Text](./docs/datasheet.png)

compile and flash program automatically
```sh
cargo run --release --example pico_blinky
# cargo run --release --example pico_pwm_blink
# cargo run --release --example adc_usb
# cargo run --release --example pico_countdown_blinky
```


## sources

boards
https://github.com/rubberduck203/stm32f3-discovery/tags
https://github.com/riscv-rust/hifive1/tags
https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico

embedded-hal
https://crates.io/crates/embedded-hal

Raspberry Silicon RP2040 microcontroller
https://crates.io/crates/rp2040-hal
note: this is re-exported by the board crate, so it does not need to be in cargo.toml if you implement the board rp-pico board crate 

usb host system
https://github.com/rust-embedded-community/usbd-serial
https://github.com/a1ien/rusb
https://github.com/rust-embedded-community/usb-device