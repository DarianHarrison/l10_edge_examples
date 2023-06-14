# l10_edge



## 5.) L10Workspace-Devel Pull

```sh
git clone https://github.com/DarianHarrison/l10_edge
git checkout l10_edge-devel
```
```sh
git pull
```
for edge tools - For creating UF2 images for the RP2040 USB Bootloader
```sh
cargo install elf2uf2-rs --locked
```
for edge tools - For flashing over the SWD pins using a supported JTAG probe
```sh
cargo install probe-run
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