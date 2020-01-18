//! Implementation function of the json related attribute macros.

use proc_macro::TokenStream;

/// Create the attribute macro form.
pub fn form_impl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, proc_mac::Form)]
        #input
    };
    output.into()
}

/// Create the attribute macro json.
pub fn json_impl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, serde::Deserialize, proc_mac::Json)]
        #input
    };
    output.into()
}

/// Create the attribute macro load.
pub fn load_impl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Deserialize, proc_mac::Load)]
        #input
    };
    output.into()
}

/// Create the attribute macro save.
pub fn save_impl(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let output = quote::quote! {
        #[derive(Debug, serde::Serialize, proc_mac::Save)]
        #input
    };
    output.into()
}
