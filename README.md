# l10_edge

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

## 4. Play the Device

1. unplug and plug device back in