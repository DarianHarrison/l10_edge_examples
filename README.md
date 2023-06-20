## l10_edge

![Alt Text](./docs/pico-datasheet.png)

Raspberry Pi Pico DataSheet
https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf


![Alt Text](./docs/ccd-datasheet.png)

3000-pixel CCD Linear Image Sensor (B/W) - ILX526A Datasheet (PDF) - Sony Corporation
https://pdf1.alldatasheet.com/datasheet-pdf/view/47503/SONY/ILX526A.html




### GPIO ADC to USB HOST

1.) Wires

![Alt Text](./docs/photoresistor.png)


2.) Run photoresistor example
```sh
cargo run --release --example gpio_in_adc
```


3.) run core to read data over usb
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
