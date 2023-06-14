# l10_edge

* timers
* pwm
* adc
* dma
* usb
* dma to usb
* usb to host


## 1. OS Tools

### On Ubuntu 22.04.x

```sh
sudo apt install -y libudev-dev
sudo apt install -y pkg-config
```

### On RHEL 9.x

```sh
sudo dnf install -y libudev-devel
sudo dnf install -y pkgconf
```

## 2. Embedded Tools

For creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```

For flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
```

## 3. Dev

1. unplug and plug device back in (while holding boostel)

2. cd to any example
```sh
cd examples/pico_l10_detector
```
3. compile and flash program automatically
```sh
cargo run --release --example pico_blinky
```
4. unplug and plug device back in


## 4. Play the Device

1. unplug and plug device back in





## additional resources

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
