# ToValue Trait for Modular Bitfields for Rust

Implementation based on the [modular-bitfield](https://crates.io/crates/modular-bitfield) crate.
Allows for the implementation of the `to_u32()` function to convert structs that implement the Specifier trait to a value.  
TODO:
- [X] `to_u32()` function to convert structs with length of 4 bytes to u32
- [ ] `to_u32()` extend this function to convert structs with length of 3 bytes to u32
- [ ] `to_u16()` function to convert structs with length of 2 bytes to u16
- [ ] `to_u8()` function to convert structs with length of 1 byte to u8
- [ ] `to_bool()` function to convert structs with length of 1 bit to bool

Fully supported in `#![no_std]` environments.

## Usage

Add this to your `cargo.toml`:

```toml
[dependencies]
modular-bitfield = "0.11.2"
modular-bitfield-to-value = { git = "https://github.com/hacknus/modular_bitfield_to_value" }
```

Add the following imports:

```rust
use modular_bitfield::bitfield;
use modular_bitfield_to_value::ToValue;
```

A basic example:

```rust
{
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct DrvStatus {
    pub test: u32,
}

let value = DrvStatus.to_u32();
}
```
