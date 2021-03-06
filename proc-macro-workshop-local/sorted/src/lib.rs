use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut c_input = proc_macro2::TokenStream::from(input.clone());
    let p_input = syn::parse_macro_input!(input as syn::ItemFn);
    println!("{:#?}", p_input);
    if let Err(e) = sort_match(&p_input) {
        c_input.extend(e.to_compile_error());
    }
    c_input.into()
}

fn sort_match(item_fn: &syn::ItemFn) -> Result<(), syn::Error> {
    let block = &item_fn.block;

    let is_sorted_apply = |attrs: &Vec<syn::Attribute>| -> bool {
        let mut result = false;
        for attr in attrs {
            for segment in &attr.path.segments {
                result = &segment.ident.to_string() == "sorted";
            }
        }
        result
    };

    for stmt in &block.stmts {
        if let syn::Stmt::Expr(syn::Expr::Match(syn::ExprMatch {
            ref attrs,
            ref arms,
            ..
        })) = stmt
        {
            let _is_sorted_apply_here = is_sorted_apply(attrs);
            let mut arm_names = Vec::new();
            for arm in arms {
                if let syn::Pat::TupleStruct(syn::PatTupleStruct { ref path, .. }) = &arm.pat {
                    let ident = &path.segments.iter().next().unwrap().ident;
                    let ident_str = ident.to_string();
                    //eprintln!("{:?}", ident_str);
                    if !arm_names.is_empty() && arm_names.last().unwrap() > &ident_str {
                        if let Err(should_be) = arm_names.binary_search(&ident_str) {
                            let error = syn::Error::new(
                                ident.span(),
                                format!(
                                    "{} should sort before {}",
                                    ident_str, arm_names[should_be]
                                ),
                            );
                            return Err(error);
                        }
                    }
                    arm_names.push(ident_str);
                };
            }
        }
    }
    Ok(())
}

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

    let check_variants_order = |item: &syn::Item| -> Result<(), syn::Error> {
        let mut var_str_vec = Vec::new();
        if let syn::Item::Enum(syn::ItemEnum { ref variants, .. }) = item {
            for variant in variants.iter() {
                let ident = variant.ident.to_string();
                if !var_str_vec.is_empty() && var_str_vec.last().unwrap() > &ident {
                    if let Err(should_be) = var_str_vec.binary_search(&ident) {
                        //eprintln!("under binart {} with search {}", should_be, &ident);
                        let error = syn::Error::new(
                            variant.ident.span(),
                            format!("{} should sort before {}", ident, var_str_vec[should_be]),
                        );
                        return Err(error);
                    }
                }
                var_str_vec.push(ident);
                //eprintln!("{:?}", var_str_vec);
            }
        }
        Ok(())
    };

    check_variants_order(&input)
}
