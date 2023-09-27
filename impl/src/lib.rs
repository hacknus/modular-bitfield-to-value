use proc_macro::TokenStream;

use quote::quote;
use syn::{DeriveInput, parse_macro_input};

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
        impl ToValue for #struct_name {
            fn to_u32(&self) -> Option<u32> {
                let array = self.into_bytes();
                match array.len() {
                    4 => Some(((array[0] as u32) << 24)
                        + ((array[1] as u32) << 16)
                        + ((array[2] as u32) << 8)
                        + ((array[3] as u32) << 0)),
                    3 => Some(((array[0] as u32) << 16)
                        + ((array[1] as u32) << 8)
                        + ((array[2] as u32) << 0)),
                    2 => Some(((array[0] as u32) << 8)
                        + ((array[1] as u32) << 0)),
                    1 => Some(((array[0] as u32) << 0)),
                    _ => None
                }
            }

            fn to_u16(&self) -> Option<u16> {
                let array = self.into_bytes();
                match array.len() {
                    2 => Some(((array[0] as u16) << 8)
                        + ((array[1] as u16) << 0)),
                    1 => Some((array[0] as u16) << 0),
                    _ => None
                }
            }

            fn to_u8(&self) -> Option<u8> {
                let array = self.into_bytes();
                match array.len() {
                    1 => Some(array[0]),
                    _ => None
                }
            }

            fn to_bool(&self) -> Option<bool> {
                let array = self.into_bytes();
                if array.len() == 1 && array[0] <= 1 {
                    Some(array[0] == 1)
                } else {
                    None
                }
            }
        }
    };

    // Return the generated implementation as a TokenStream.
    TokenStream::from(expanded)
}