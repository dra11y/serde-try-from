use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

/// Derives both:
/// - `TryFrom<serde_json::Value> for Struct`
/// - `TryFrom<Struct> for serde_json::Value`
///
/// Struct must implement both `serde::Serialize` and `serde::de::DeserializeOwned`.
#[proc_macro_derive(TryFrom)]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let try_from_value = generate_try_from_value(&input.ident);
    let try_from_struct = generate_try_from_struct(&input.ident);
    TokenStream::from(quote! {
        #try_from_value
        #try_from_struct
    })
}

/// Derives `TryFrom<Struct> for serde_json::Value`.
/// Struct must implement `serde::Serialize`.
#[proc_macro_derive(TryFromSe)]
pub fn derive_try_from_se(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(generate_try_from_struct(&input.ident))
}

/// Derives `TryFrom<serde_json::Value> for Struct`.
/// Struct must implement `serde::de::DeserializeOwned`.
#[proc_macro_derive(TryFromDe)]
pub fn derive_try_from_de(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(generate_try_from_value(&input.ident))
}

fn generate_try_from_struct(name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::TryFrom<#name> for serde_json::Value
        where
            #name: serde::Serialize,
        {
            type Error = serde_json::Error;

            fn try_from(value: #name) -> Result<Self, Self::Error> {
                serde_json::to_value(value)
            }
        }
    }
}

fn generate_try_from_value(name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::TryFrom<serde_json::Value> for #name
        where
            #name: serde::de::DeserializeOwned,
        {
            type Error = serde_json::Error;

            fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
                serde_json::from_value(value)
            }
        }
    }
}
