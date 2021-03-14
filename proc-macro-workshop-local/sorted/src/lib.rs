use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut p_input = syn::parse_macro_input!(input as syn::ItemFn);
    remove_custom_attr(&mut p_input);
    //eprintln!("{:#?}", p_input);
    let mut c_input = quote! {#p_input};
    if let Err(e) = sort_match(&p_input) {
        c_input.extend(e.to_compile_error());
    }
    c_input.into()
}

struct RemoveAttr;

impl VisitMut for RemoveAttr {
    fn visit_expr_match_mut(&mut self, i: &mut syn::ExprMatch) {
        i.attrs.clear();
    }
}

fn remove_custom_attr(input: &mut syn::ItemFn) {
    RemoveAttr.visit_item_fn_mut(input);
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

            let get_ident_name = |path: &syn::Path| -> String {
                let mut data = Vec::new();
                path.segments.iter().for_each(|segment| {
                    data.push(segment.ident.to_string());
                });
                let full_name = data.join("::").to_string();
                full_name
            };

            for arm in arms {
                eprintln!("{:#?}", arm);
                if let syn::Pat::TupleStruct(syn::PatTupleStruct { ref path, .. }) = &arm.pat {
                    let ident_str = get_ident_name(&path);
                    if !arm_names.is_empty() && arm_names.last().unwrap() > &ident_str {
                        if let Err(should_be) = arm_names.binary_search(&ident_str) {
                            let error = syn::Error::new_spanned(
                                path,
                                format!(
                                    "{} should sort before {}",
                                    ident_str, arm_names[should_be]
                                ),
                            );
                            return Err(error);
                        }
                    }
                    arm_names.push(ident_str);
                } else {
                    let error = syn::Error::new_spanned(&arm.pat, "unsupported by #[sorted]");
                    return Err(error);
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
    //println!("{:#?}", p_input);
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
