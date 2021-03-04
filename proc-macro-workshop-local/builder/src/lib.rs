use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::Data::Struct;
use syn::{parse_macro_input, DeriveInput, Field, Type, TypePath};

#[proc_macro_derive(Builder, attributes(builder))]
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

    fn get_builder_attr_name(stream: proc_macro2::TokenStream) -> Option<String> {
        let token_tree = &mut stream.into_iter();
        for token in token_tree {
            if let proc_macro2::TokenTree::Group(ref g) = token {
                return get_builder_attr_name(g.stream());
            }
            if let proc_macro2::TokenTree::Literal(l) = token {
                return Some(l.to_string().replace("\"", ""));
            }
        }
        None
    }

    let is_builder = |field: &Field| -> (bool, Option<String>) {
        let mut result = false;
        let mut builder_name = None;
        let attr = &field.attrs.iter().next();
        if attr.is_some() {
            let attr = attr.unwrap();
            builder_name = get_builder_attr_name(attr.tokens.clone());
            let segments = &mut attr.path.segments.iter();
            let segments = segments.next();
            if segments.is_some() {
                result = if segments.unwrap().ident == "builder" {
                    true
                } else {
                    false
                };
            }
        }
        return (result, builder_name);
    };

    let method = fields.iter().map(|field| {
        let (_, attr_name) = is_builder(&field);
        let name = field.ident.as_ref();
        let field_name = name.unwrap().to_string();
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
            let attr_name = attr_name.unwrap_or(String::from(""));
            if attr_name == field_name {
                let attr_ident = Ident::new(&attr_name, Span::call_site());
                quote! {
                    fn #attr_ident(&mut self, #attr_ident: String) -> &mut Self {
                        let mut vec = self.#name.take().unwrap_or(Vec::new());
                        vec.push(#attr_ident);
                        self.#name = Some(vec);
                        self
                    }
                }
            } else if !attr_name.is_empty() {
                let attr_ident = Ident::new(&attr_name, Span::call_site());
                quote! {
                    fn #attr_ident(&mut self, #attr_ident: String) -> &mut Self {
                        let mut vec = self.#name.take().unwrap_or(Vec::new());
                        vec.push(#attr_ident);
                        self.#name = Some(vec);
                        self
                    }

                    fn #name(&mut self, #name: #ty) -> &mut Self {
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
        }
    });

    let field_with_default = fields.iter().map(|field| {
        let name = field.ident.as_ref();
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

    let new_builder_fields = fields.iter().map(|field| {
        let name = field.ident.as_ref();
        let (is_builder, _) = is_builder(&field);
        if is_builder {
            quote! {
                #name: Some(Vec::new())
            }
        } else {
            quote! {
                #name: None
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
                    #(#new_builder_fields,)*
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
