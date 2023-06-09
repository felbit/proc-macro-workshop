use proc_macro::TokenStream;
use quote::quote;
use syn::{self, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let struct_ident = &ast.ident;
    let builder_struct_ident =
        Ident::new(&format!("{}Builder", &struct_ident), struct_ident.span());

    let gen = quote! {
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

        pub struct #builder_struct_ident {
            executable: Option<String>,
            args: Option<Vec<String>>,
            env: Option<Vec<String>>,
            current_dir: Option<String>,
        }

        use std::error::Error;

        impl #builder_struct_ident {
            pub fn executable(&mut self, executable: String) -> &mut Self {
                self.executable = Some(executable);
                self
            }

            pub fn args(&mut self, args: Vec<String>) -> &mut Self {
                self.args = Some(args);
                self
            }

            pub fn env(&mut self, env: Vec<String>) -> &mut Self {
                self.env = Some(env);
                self
            }

            pub fn current_dir(&mut self, current_dir: String) -> &mut Self {
                self.current_dir = Some(current_dir);
                self
            }

            pub fn build(&mut self) -> Result<#struct_ident, Box<dyn Error>> {
                Ok(#struct_ident {
                    executable: self.executable.clone().ok_or("executable not set")?,
                    args: self.args.clone().ok_or("args not set")?,
                    env: self.env.clone().ok_or("env not set")?,
                    current_dir: self.current_dir.clone().ok_or("current_dir not set")?,
                })
            }
        }
    };

    gen.into()
}
