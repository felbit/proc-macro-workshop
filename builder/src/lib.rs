use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as syn::DeriveInput);

    let struct_ident = &ast.ident;
    let builder_name = format!("{}Builder", &struct_ident);
    let builder_struct_ident = syn::Ident::new(&builder_name, struct_ident.span());

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = ast.data
    {
        named
    } else {
        panic!("!")
    };

    let option_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        if ty_is_option(&f) {
            quote!(#name: #ty)
        } else {
            quote!(#name: std::option::Option<#ty>)
        }
    });

    let builder_methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;

        if ty_is_option(&f) {
            let inner_ty = extract_inner_type_ident(&f).unwrap();
            quote! {
                pub fn #name(&mut self, #name: #inner_ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                pub fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let builder_build_method_values = fields.iter().map(|f| {
        let name = &f.ident;
        if ty_is_option(&f) {
            quote!(#name: self.#name.clone())
        } else {
            quote!(#name: self.#name.clone().ok_or("#name not found")?)
        }
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
            #(#builder_methods)*

            pub fn build(&self) -> Result<#struct_ident, Box<dyn std::error::Error>> {
                Ok(#struct_ident {
                    #(#builder_build_method_values,)*
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

fn ty_is_option(f: &syn::Field) -> bool {
    if let syn::Type::Path(ref p) = f.ty {
        return p.path.segments.last().unwrap().ident.to_string() == "Option";
    }
    false
}

fn extract_inner_type_ident(f: &syn::Field) -> Option<syn::Ident> {
    if let syn::Type::Path(ref p) = f.ty {
        let args = &p.path.segments.last().unwrap().arguments;
        if let syn::PathArguments::AngleBracketed(ref ab_args) = args {
            let arg = &ab_args.args.first().unwrap();
            if let syn::GenericArgument::Type(syn::Type::Path(ref p)) = arg {
                return Some(p.path.segments.first().unwrap().ident.clone());
            }
        }
    }
    None
}
