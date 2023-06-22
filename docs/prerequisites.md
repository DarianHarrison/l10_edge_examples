# Lion10 Edge Prerequisites

* [X] 0. prerequisites
* [X] 1. std
* [X] 2. no_std

## 0. prerequisites


### A) Operating System

Any of the following Operating Systems are supported:

* Ubuntu 22.04 or higher
* RHEL 9 or higher


### B) Rust Programming Language

* Rust 1.31.0 or higher
```
https://www.rust-lang.org/tools/install
```

### C) Lion10 Edge (no_std)

* lion10-edge-0.0.1 or higher
```
https://github.com/DarianHarrison/l10_edge/releases
```

### D) Lion10 Core (std) Software

* lion10-core-0.0.1 or higher
```
https://github.com/DarianHarrison/l10_core/releases
```

## 1. std

### A) Update Rust

```sh
rustup default stable
rustup update
rustup self update
rustup update stable
rustc --version --verbose
```

## 2. no_std

### A) Download target architectures
```sh
rustup target add thumbv6m-none-eabi # rp-rs
rustup target add thumbv7em-none-eabihf # stm32f3-discovery & nucleo-l432kc
rustup target add riscv32imac-unknown-none-elf # board-hifive1-revb
```

### B) Tool for creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```

### C) Tool for flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
```

### D) Allow USB device access as ```non-root```

#### D.1) find the usb you want to configure non-root access
```sh
lsusb
```
Bus 001 Device 016: ID ```16c0:27dd``` Van Ooijen Technische Informatica CDC-ACM class devices (modems)


#### D.2) add this line to the following file: ```/etc/udev/rules.d/70-st-link.rules```
```sh  
ATTRS{idVendor}=="16c0", ATTRS{idProduct}=="27dd", TAG+="uaccess"
```

#### D.3) reload udev rules
```sh
sudo udevadm control --reload-rules
```

#### D.4)
```
plug board back out and back in
```