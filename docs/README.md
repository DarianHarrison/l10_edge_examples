# l10_edge - Setup

* [ ] 0. prerequisites
* [X] 1. std
* [X] 2. no_std


## 0. prerequisites

## 1. std
```sh
rustup default stable
rustup update
rustup self update
rustup update stable
rustc --version --verbose
```

## 2. no_std

a) download target architectures for the following boards:

* rp-rs
* nucleo-l432kc
* stm32f3-discovery
* board-hifive1-revb

```sh
rustup target add thumbv6m-none-eabi # rp-rs
rustup target add thumbv7em-none-eabihf # stm32f3-discovery & nucleo-l432kc
rustup target add riscv32imac-unknown-none-elf # board-hifive1-revb
```

b) tool for creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```

c) For flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
```

d) Allow USB device access as ```non-root```

d.1) find the usb you want to configure non-root access
```sh
lsusb
```
Bus 001 Device 016: ID ```16c0:27dd``` Van Ooijen Technische Informatica CDC-ACM class devices (modems)


d.2) add this line to the following file: ```/etc/udev/rules.d/70-st-link.rules```
```sh  
ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="27dd", TAG+="uaccess"
```

d.3) reload udev rules
```sh
sudo udevadm control --reload-rules
```

d.4)
```
plug board back out and back in
```