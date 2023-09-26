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

/// The ToValue trait defines the to_u32() function
pub trait ToValue {
    /// Converts a 4 byte struct to a u32,
    /// returns None if the struct is not of length 32 bits
    fn to_u32(&self) -> Option<u32>;
}