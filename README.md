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
git clone 
git checkout devel
```

try to commit and push
```sh
git commit
git push
git pull
```

## 4.) Run no_std

1. look at available board examples
```
ls examples/pico_l10_detector/examples
```
2. unplug and plug device back in (while holding boostel)

3. cd to any example
```sh
cd examples/pico_l10_detector
```
4. compile and flash program automatically
```sh
cargo run --release --example pico_blinky
# cargo run --release --example pico_pwm_blink
```

## 5.) Play the Device

1. unplug and plug device back in