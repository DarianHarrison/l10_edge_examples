## l10_edge

![Alt Text](./docs/pico-datasheet.png)

Raspberry Pi Pico DataSheet
https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf


### GPIO ADC to USB HOST

1.) Wires

![Alt Text](./docs/photoresistor.png)


2.) Edge
https://github.com/DarianHarrison/l10_edge
```sh
cargo run --release --example gpio_in_adc
```

3.) Core
https://github.com/DarianHarrison/l10_core
```sh
cargo run 0x16c0 0x27dd
```

## sources

// board-crates
https://crates.io/crates/embedded-hal
https://crates.io/crates/rp2040-hal
https://crates.io/crates/rp-pico

// arm-crates
https://crates.io/crates/cortex-m

// core
usb host system
https://github.com/rust-embedded-community/usbd-serial
https://github.com/a1ien/rusb
https://github.com/rust-embedded-community/usb-device
