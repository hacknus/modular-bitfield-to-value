//! Implementation based on the modular-bitfield crate.
//! Allows for the implementation for structs that implement the Specifier trait.
//!
//! Usage:
//! ```rust
//! use modular_bitfield::bitfield;
//! use modular_bitfield_to_value::ToValue;
//!
//! #[bitfield(bits = 32)]
//! #[derive(ToValue)]
//! pub struct DrvStatus {
//!     pub test: u32,
//! }
//! ```
//!
//!
#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]


pub use modular_bitfield_to_value_impl::{
    ToValue,
};

/// The ToValue trait defines the conversion functions
pub trait ToValue {
    /// Converts a struct to a u32 (little endian),
    /// returns None if the struct is longer than 32 bits
    fn to_u32_le(&self) -> Option<u32>;

    /// Converts a struct to a u32,
    /// returns None if the struct is longer than 32 bits
    fn to_u32(&self) -> Option<u32>;

    /// Converts a struct to a u16,
    /// returns None if the struct is longer than 16 bits
    fn to_u16(&self) -> Option<u16>;

    /// Converts a struct to a u8,
    /// returns None if the struct is not of length 8 bits
    fn to_u8(&self) -> Option<u8>;

    /// Converts a struct to a bool,
    /// returns None if the struct is not of length 1 bit
    fn to_bool(&self) -> Option<bool>;
}