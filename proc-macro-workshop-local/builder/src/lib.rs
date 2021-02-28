use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Data::Struct;
use syn::{parse_macro_input, DeriveInput, Field, Type, TypePath};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parse_input = parse_macro_input!(input as DeriveInput);
    eprintln!("{:#?}", parse_input);
    let fields = if let Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = parse_input.data
    {
        named
    } else {
        panic!("No field found in command")
    };

    let is_field_optional = |field: &Field| {
        let ty = &field.ty;
        if let Type::Path(TypePath { ref path, .. }) = ty {
            path.segments
                .iter()
                .any(|path_seg| path_seg.ident == "Option")
        } else {
            false
        }
    };

    let method = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        let ty = &field.ty;
        let is_op = is_field_optional(field);
        if is_op {
            quote! {
                fn #name(&mut self, #name: String) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        } else {
            quote! {
                fn #name(&mut self, #name: #ty) -> &mut Self {
                    self.#name = Some(#name);
                    self
                }
            }
        }
    });

    let field_with_default = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        let ty = &field.ty;
        let is_op = is_field_optional(field);
        if !is_op {
            quote! {
                #name: self.#name.take().unwrap().clone()
            }
        } else {
            quote! {
                #name: self.#name.clone()
            }
        }
    });

    let op_builder_fields = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        let ty = &field.ty;
        if is_field_optional(field) {
            quote! {
                #name: #ty
            }
        } else {
            quote! {
                #name: Option<#ty>
            }
        }
    });

    let name = parse_input.ident;
    let command_builder = Ident::new(&format!("{}Builder", name), Span::call_site());
    let expansion = quote! {
        impl #name {
            fn builder() -> #command_builder {
                #command_builder {
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None
                }
            }
        }

        struct #command_builder {
           #(#op_builder_fields,)*
        }

        impl #command_builder {
            fn build(&mut self) -> std::result::Result<#name, ()> {
                Ok(#name {
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
