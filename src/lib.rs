use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, parse_macro_input};

#[proc_macro_derive(TryFrom)]
pub fn derive_try_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(generate_try_from(&input.ident))
}

#[proc_macro_derive(Into)]
pub fn derive_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    TokenStream::from(generate_into(&input.ident))
}

#[proc_macro_derive(TryFromInto)]
pub fn derive_try_from_into(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let try_from = generate_try_from(&input.ident);
    let into = generate_into(&input.ident);
    TokenStream::from(quote! {
        #try_from
        #into
    })
}

fn generate_try_from(name: &Ident) -> proc_macro2::TokenStream {
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

fn generate_into(name: &Ident) -> proc_macro2::TokenStream {
    quote! {
        impl std::convert::Into<serde_json::Value> for #name
        where
            #name: serde::Serialize,
        {
            fn into(self) -> serde_json::Value {
                serde_json::to_value(self).unwrap_or_else(|e| {
                    panic!("Failed to convert {} into serde_json::Value: {}", stringify!(#name), e)
                })
            }
        }
    }
}
