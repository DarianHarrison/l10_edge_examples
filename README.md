# l10_edge

* timers
* adc
* usb
* pwm

1. 
```bash
cargo run --release --example adc_usb
```

2. 
```bash
sudo apt install -y screen
sudo screen /dev/ttyACM0 115200
```
```bash
CTRL+A 
k
y
```



## sources

pico board crate
https://crates.io/crates/rp-pico

Raspberry Silicon RP2040 microcontroller
https://crates.io/crates/rp2040-hal
note: this is re-exported by the board crate, so it does not need to be in cargo.toml if you implement the board rp-pico board crate 

embedded-hal
https://crates.io/crates/embedded-hal


## reference

![Alt Text](./docs/datasheet.png)