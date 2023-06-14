* timers
* pwm
* adc
* dma
* usb
* dma to usb
* usb to host

## 10. additional resources

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


# https://github.com/rubberduck203/stm32f3-discovery/tags
# https://github.com/riscv-rust/hifive1/tags
# https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico
# git clone latest discovery board
# git clone latest pi-pico board
# git clone latest si-labs board
# git clone latest stmf32 board




## 6.) L10Workspace-Devel Run

1. Edge (first terminal)
```sh
cargo edge run pico
cargo edge run stmf32
```
```sh
cargo run --release --example adc_usb
```

2. Host (second terminal)
test that you can run host project
```sh
cargo run -p hello_world
```
```sh
cargo core run sub_binary_to_tikv
```
```sh
cargo run 0x16c0 0x27dd
```

3. Dashboard (third terminal)
```
Grafana
Kibana
Other
```