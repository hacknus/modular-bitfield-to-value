# ToValue Trait for Modular Bitfields for Rust

[![crates.io](https://img.shields.io/crates/v/modular-bitfield-to-value.svg)](https://crates.io/crates/modular-bitfield-to-value)
[![Docs](https://docs.rs/modular-bitfield-to-value/badge.svg)](https://docs.rs/modular-bitfield-to-value)
[![Rust](https://github.com/hacknus/modular-bitfield-to-value/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/modular-bitfield-to-value/actions/workflows/rust.yml)  

Implementation based on the [modular-bitfield](https://crates.io/crates/modular-bitfield) crate.
Allows for the implementation of the `to_u32()` function to convert structs that implement the Specifier trait to a value.  

Features:
- [X] `to_u32()` function to convert structs with length of max 4 bytes to u32
- [X] `to_u16()` function to convert structs with length of max 2 bytes to u16
- [X] `to_u8()` function to convert structs with length of max 1 byte to u8
- [X] `to_bool()` function to convert structs with length of 1 bit to bool  

Todo:
- [ ] implement little-endian conversion?
- [ ] maybe functions to return `u64` and `u128` could be implemented...

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
#[bitfield(bits = 32)]
#[derive(ToValue)]
pub struct Field {
    pub test: u32,
}


{
    let field = Field::from_bytes(0x100C3_u32.to_be_bytes());
    let value = field.to_u32().unwrap();
    assert!(0x100C3 == value, "constructed = {}, to_u32() = {}", 0x100C3, 0x100C3);
}
```
