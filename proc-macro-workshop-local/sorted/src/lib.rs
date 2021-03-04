use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let p_input = syn::parse_macro_input!(input as syn::Item);
    let _ = args;
    let _ = input;
    println!("{:#?}", p_input);
    let is_item_enum = |item: &syn::Item| {
        if let syn::Item::Enum(syn::ItemEnum { .. }) = item {
            return true;
        }
        false
    };
    if !is_item_enum(&p_input) {
        let error = syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        )
        .into_compile_error();
        return proc_macro::TokenStream::from(error);
    }

    TokenStream::new()
}
