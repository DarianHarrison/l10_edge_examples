# l10_edge

## TODOS
* [ ] try to split rx receive and tx transmit on USB (USB PHY)
* [ ] set up an external clock with PIO
* [ ] clock dividers and baud rates with PIO
* [ ] PWM Period examples
* [ ] autopush with PIO
* [ ] sample external gpio pins with PIO
* [ ] access default usb counter instead of timer for the wrapping CDC 
* [ ] access default usb counter instead of timer
* [ ] interpolator test

## 0. Prerequisites

* 0. 
* 1. 

## 1. Examples

### A) Output

#### A.0) Wiring


![Alt Text](./docs/led.png)


#### A.1) GPIO output LED Blinky
```sh
cargo run --release --example gpio_out_blinky
```

#### A.2) GPIO output - LED PWM
```sh
cargo run --release --example gpio_out_pwd
```

#### A.3) PIO output - LED PWM
```sh
cargo run --release --example pio_out_pwm
```

### B) Input

#### B.0) Wiring


![Alt Text](./docs/photoresistor.png)


#### B.1) GPIO input - Photoresistor ADC to USB

a) on first terminal
```sh
cargo run --release --example gpio_in_adc_to_usb
```

b) on another terminal - read data over usb
```sh
git clone https://github.com/DarianHarrison/l10_core
cd l10_core
cargo run 0x16c0 0x27dd
```