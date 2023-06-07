use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let struct_ident = &ast.ident;
    let builder_struct_ident = Ident::new(&format!("{}Builder", &struct_ident), Span::call_site());

    let gen = quote! {
        pub struct #builder_struct_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        impl #struct_ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
    };

    gen.into()
}
