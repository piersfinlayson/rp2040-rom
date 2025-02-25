# rp2040-rom

A lightweight Rust crate providing safe access to Raspberry Pi RP2040 ROM functions.

## Features

- Access to RP2040 ROM functions from Rust
- Currently implemented:
  - `reset_usb_boot`: Reset the chip and enter USB bootloader (DFU) mode

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rp2040-rom = "0.1.0"
```

### Example

```rust
use rp2040_rom::ROM;

// Reset into USB bootloader (DFU) mode
unsafe {
    ROM::reset_usb_boot(0, 0);
}
```

## Safety

All ROM functions are marked as `unsafe` because:

1. They involve direct hardware manipulation
2. They can reset the device
3. They require specific hardware (RP2040)

## Documentation

For more details on the RP2040 ROM functions, see the [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf).

## License

Licensed under MIT License.