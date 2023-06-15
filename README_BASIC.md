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

```sh
lsusb
```
Bus 001 Device 016: ID 16c0:27dd Van Ooijen Technische Informatica CDC-ACM class devices (modems)


Set devices without root privilege
```sh
sudo sh -c "cat << 'EOF' >> /etc/udev/rules.d/70-st-link.rules
ATTRS{idVendor}=="1366", ATTRS{idProduct}=="1051", TAG+="uaccess"
EOF"
```
```sh
sudo udevadm control --reload-rules
```
verify
```
plug board back out and back in
```

```sh
lsusb
ls -l /dev/bus/usb/001/023
```
crw-rw-r--+ 1 root root 189, 22 Jun 14 22:57 /dev/bus/usb/001/023


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
# cargo run --release --example adc_usb
```

## 5.) Play the Device

1. unplug and plug device back in (without holding boostel)