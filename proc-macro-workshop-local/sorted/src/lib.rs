use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let p_input = syn::parse_macro_input!(input as syn::Item);
    let _ = args;
    let _ = input;
    eprintln!("{:#?}", p_input);

    TokenStream::new()
}
