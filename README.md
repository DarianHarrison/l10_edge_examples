# l10_edge-devel

- [ ] 1. Prereqs.
- [ ] 2. Prereqs.
- [ ] 3. Prereqs.
- [ ] 4. Prereqs.
- [ ] 5. Prereqs.
- [ ] 6. Install Rust no_std
- [ ] 7. Install Embedded Tools
- [ ] 8. l10_edge-devel
- [ ] 9. Run no_std
- [ ] 10. Play the Device


## 6.) Rust no_std

update your rust installation and ensure you are on the stable build
```sh
cd ~/Desktop/l10_core
rustup default stable ; rustup update ; rustup self update ; rustup update stable ; rustc --version --verbose
```

download the compilation target architectures
```sh
# stm32f3-discovery, board-hifive1-revb, rp-rs, nucleo-l432kc
rustup target add thumbv7em-none-eabihf ; rustup target add riscv32imac-unknown-none-elf ; rustup target add thumbv6m-none-eabi ; rustup target add thumbv7em-none-eabihf
```

## 7.) Embedded Tools

For creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```

For flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
```

## 8.) l10_edge-devel
```sh
git clone https://github.com/DarianHarrison/l10_edge
git checkout l10_edge-devel
```

try to commit and push
```sh
git commit
git push
git pull
```

## 9.) Run no_std

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

## 10.) Play the Device

1. unplug and plug device back in