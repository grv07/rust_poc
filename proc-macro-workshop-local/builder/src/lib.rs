use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Data::Struct;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parse_input = parse_macro_input!(input as DeriveInput);
    let fields = if let Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = parse_input.data
    {
        named
    } else {
        panic!("No field found in command")
    };
    let method = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        let ty = &field.ty;
        quote! {
            fn #name(&mut self, #name:#ty) -> &mut Self {
                self.#name = #name;
                self
            }
        }
    });
    let field_with_default = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        quote! {
            #name: self.#name.clone()
        }
    });

    let name = parse_input.ident;
    let command_builder = Ident::new(&format!("{}Builder", name), Span::call_site());
    let expansion = quote! {
        impl #name {
            fn builder() -> #command_builder {
                #command_builder {
                    executable: "".to_string(),
                    args: vec![],
                    env: vec![],
                    current_dir: "".to_string()
                }
            }
        }

        struct #command_builder {
           #fields
        }

        impl #command_builder {
            fn build(&self) -> std::result::Result<#command_builder, ()> {
                Ok(#command_builder {
                    #(#field_with_default,)*
                })
            }
    }
        impl #command_builder {
            #(#method)*
        }
    };
    TokenStream::from(expansion)
}
