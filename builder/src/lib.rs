use proc_macro::TokenStream;
use syn::{self, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::new()
}
