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

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Macro derivation to implement ToValue for a struct that implements Specifier
/// and is of length 32 bits
/// e.g. it was annotated with #[bitfield(bits = 32]
#[proc_macro_derive(ToValue)]
pub fn derive_to_value(input: TokenStream) -> TokenStream {
    // Parse the input into a syntax tree.
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct for which we are implementing the trait.
    let struct_name = &input.ident;

    // Generate the implementation of the ToValue trait.
    let expanded = quote! {
        /// The ToValue trait defines the to_u32() function
        pub trait ToValue {
            /// Converts a 4 byte struct to a u32,
            /// returns None if the struct is not of length 32 bits
            fn to_u32(&self) -> Option<u32>;
        }

        impl ToValue for #struct_name {
            fn to_u32(&self) -> Option<u32> {
                let array = self.into_bytes();
                if array.len() != 4 {
                    None
                } else {
                    Some(((array[0] as u32) << 24)
                        + ((array[1] as u32) << 16)
                        + ((array[2] as u32) << 8)
                        + ((array[3] as u32) << 0))
                }
            }
        }
    };

    // Return the generated implementation as a TokenStream.
    TokenStream::from(expanded)
}