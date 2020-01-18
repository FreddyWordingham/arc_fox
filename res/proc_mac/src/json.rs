//! Implementation function of the json related procedural macros.

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

/// Implement `Save` and `Load` traits using json parsing.
pub fn impl_form_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    let output = quote! {
        impl arc::file::io::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                arc::file::io::as_json(self, path);
            }
        }

        impl arc::file::io::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                arc::file::io::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}

/// Implement `Save` and `Load` traits using json parsing.
pub fn impl_json_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    let output = quote! {
        impl crate::file::io::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                crate::file::io::as_json(self, path);
            }
        }

        impl crate::file::io::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                crate::file::io::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}

/// Implement the `Save` trait using json parsing.
pub fn impl_save_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    let output = quote! {
        impl crate::file::io::Save for #name {
            #[inline]
            fn save(&self, path: &std::path::Path) {
                crate::file::io::as_json(self, path);
            }
        }
    };

    TokenStream::from(output)
}

/// Implement the `Load` trait using json parsing.
pub fn impl_load_macro(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;

    let output = quote! {
        impl crate::file::io::Load for #name {
            #[inline]
            fn load(path: &std::path::Path) -> Self {
                crate::file::io::from_json(path)
            }
        }
    };

    TokenStream::from(output)
}
