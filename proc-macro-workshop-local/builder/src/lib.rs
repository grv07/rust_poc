use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let parse_input = parse_macro_input!(input as DeriveInput);
    println!("{:#?}", parse_input);
    let name = parse_input.ident;
    let builder_name = format!("{}Builder", name);
    println!("{}", builder_name);

    let command_builder = Ident::new(&format!("{}Builder", name), Span::call_site());
    let expansion = quote! {
        impl #name {
            fn builder() -> #command_builder {
                #command_builder {}
            }
        }

        struct #command_builder {
        }
        impl #command_builder {
            fn executable(&self, executable:String) {}
            fn env(&self, env: Vec<String>) {}
            fn current_dir(&self, current_dir: String) {}
            fn args(&self, args: Vec<String>) {}
        }
    };
    TokenStream::from(expansion)
}
