use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_ident = &ast.ident;
    let builder_struct_ident =
        syn::Ident::new(&format!("{}Builder", &struct_ident), struct_ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("!")
    };

    let option_fields = fields.iter().map(|field| {
        let name = &field.ident;
        let ty = &field.ty;
        quote!(#name: std::option::Option<#ty>)
    });

    let builder_fns = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, #name: #ty) -> &mut Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    let build_vals = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(#name: self.#name.clone().ok_or("#name not found")?)
    });

    let builder_defaults = fields.iter().map(|f| {
        let name = &f.ident;
        quote!(#name: None)
    });

    let gen = quote! {
        pub struct #builder_struct_ident {
            #(#option_fields,)*
        }
        impl #builder_struct_ident {
            #(#builder_fns)*

            pub fn build(&mut self) -> Result<#struct_ident, Box<dyn ::std::error::Error>> {
                Ok(#struct_ident {
                    #(#build_vals,)*
                })
            }
        }
        impl #struct_ident {
            pub fn builder() -> #builder_struct_ident {
                #builder_struct_ident {
                    #(#builder_defaults,)*
                }
            }
        }
    };

    gen.into()
}
