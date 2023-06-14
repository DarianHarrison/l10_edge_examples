# l10_edge-devel

- [ ] 0. Prereqs: steps 1-5 of **l10_core-devel**
- [ ] 1. Install Rust no_std
- [ ] 2. Install Embedded Tools
- [ ] 3. l10_edge-devel
- [ ] 4. Run no_std
- [ ] 5. Play the Device


## 1.) Rust no_std

update your rust installation and ensure you are on the stable build
```sh
cd ~/Desktop/l10_edge
rustup default stable ; rustup update ; rustup self update ; rustup update stable ; rustc --version --verbose
```

download the compilation target architectures
```sh
# stm32f3-discovery, board-hifive1-revb, rp-rs, nucleo-l432kc
rustup target add thumbv7em-none-eabihf ; rustup target add riscv32imac-unknown-none-elf ; rustup target add thumbv6m-none-eabi ; rustup target add thumbv7em-none-eabihf
```

## 2.) Embedded Tools

For creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```

For flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
```

## 3.) l10_edge-devel

```sh
cd ~/Desktop
git clone https://github.com/DarianHarrison/l10_edge
cd l10_edge
git checkout devel
git branch
```

try to commit and push
```sh
git commit
git push
git pull
```

## 4.) Run no_std

1. unplug and plug device back in (while holding boostel)

2. look at available board examples
```
ls examples/pico_l10_detector/examples
```
3. unplug and plug device back in (while holding boostel)

4. cd to any example
```sh
cd examples/pico_l10_detector
```
5. compile and flash program automatically
```sh
cargo run --release --example pico_blinky
# cargo run --release --example pico_pwm_blink
# cargo run --release --example adc_usb
```

6. TODOS:
- [] timers
- [] pwm
- [] adc
- [] dma
- [] usb
- [] dma to usb
- [] usb to host


![Alt Text](./docs/datasheet.png)


## 5.) Play the Device





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


# https://github.com/rubberduck203/stm32f3-discovery/tags
# https://github.com/riscv-rust/hifive1/tags
# https://github.com/rp-rs/rp-hal-boards/tree/main/boards/rp-pico
# git clone latest discovery board
# git clone latest pi-pico board
# git clone latest si-labs board
# git clone latest stmf32 board