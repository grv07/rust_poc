use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn sorted(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut c_input = proc_macro2::TokenStream::from(input.clone());
    let p_input = syn::parse_macro_input!(input as syn::Item);
    println!("{:#?}", p_input);
    if let Err(e) = sorted_variants(p_input) {
        c_input.extend(e.to_compile_error());
    }
    c_input.into()
}

fn sorted_variants(input: syn::Item) -> Result<(), syn::Error> {
    // Checks if macro_attr is on an enum only.
    let is_item_enum = |item: &syn::Item| {
        if let syn::Item::Enum(syn::ItemEnum { .. }) = item {
            return true;
        }
        false
    };
    // If Item is not enum then show an compile time error.
    if !is_item_enum(&input) {
        let error = syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected enum or match expression",
        );
        return Err(error);
    }

    let variants = |item: &syn::Item| -> (Vec<syn::Ident>, Vec<String>) {
        let mut var_vec = Vec::new();
        let mut var_str_vec = Vec::new();
        if let syn::Item::Enum(syn::ItemEnum { ref variants, .. }) = item {
            for variant in variants.iter() {
                var_vec.push(variant.ident.clone());
                var_str_vec.push(variant.ident.to_string());
            }
        }
        (var_vec, var_str_vec)
    };

    let (field_ident, field_name) = variants(&input);
    let mut name_vec = field_name.clone();
    name_vec.sort();

    if name_vec != field_name {
        for name in name_vec.iter().enumerate() {
            if &field_name[name.0] != name.1 {
                let span_idx = field_name.iter().position(|r| r == name.1).unwrap();
                let error = syn::Error::new(
                    field_ident[span_idx].span(),
                    format!("{} should sort before {}", name.1, field_name[name.0]),
                );
                return Err(error);
            }
        }
    }
    Ok(())
}
