use proc_macro::TokenStream;
use quote::quote;
use syn::visit_mut::VisitMut;

#[proc_macro_attribute]
pub fn check(args: TokenStream, input: TokenStream) -> TokenStream {
    let _ = args;
    let mut p_input = syn::parse_macro_input!(input as syn::ItemFn);
    //remove_custom_attr(&mut p_input);
    let mut ra = RemoveAttr::default();
    ra.visit_item_fn_mut(&mut p_input);
    let mut c_input = quote! {#p_input};
    if let Some(e) = ra.errors.first() {
        c_input.extend(e.to_compile_error());
    }
    c_input.into()
}

#[derive(Default)]
struct RemoveAttr {
    errors: Vec<syn::Error>,
}

impl RemoveAttr {
    fn get_path_str(path: &syn::Path) -> String {
        return path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join("::");
    }

    fn get_ident_path(pat: &syn::Pat) -> Option<&syn::Path> {
        let path = match pat {
            syn::Pat::TupleStruct(syn::PatTupleStruct { ref path, .. }) => Some(path),
            syn::Pat::Ident(syn::PatIdent { ref subpat, .. }) => {
                if subpat.is_some() {
                    let (_, ref pat) = subpat.as_ref().unwrap();
                    return Self::get_ident_path(&pat);
                }
                None
            }
            syn::Pat::Path(syn::PatPath { ref path, .. }) => Some(path),
            _ => None,
        };
        path
    }
}
impl VisitMut for RemoveAttr {
    fn visit_expr_match_mut(&mut self, i: &mut syn::ExprMatch) {
        i.attrs.clear();

        let mut arm_names = Vec::new();
        for arm in &i.arms {
            if let Some(path) = Self::get_ident_path(&arm.pat) {
                let ident_str = Self::get_path_str(&path);
                if !arm_names.is_empty() && arm_names.last().unwrap() > &ident_str {
                    if let Err(should_be) = arm_names.binary_search(&ident_str) {
                        let error = syn::Error::new_spanned(
                            path,
                            format!("{} should sort before {}", ident_str, arm_names[should_be]),
                        );
                        self.errors.push(error);
                    }
                }
                arm_names.push(ident_str);
            } else {
                if let syn::Pat::Wild(syn::PatWild { .. }) | syn::Pat::Ident(syn::PatIdent { .. }) =
                    &arm.pat
                {
                    continue;
                }
                let error = syn::Error::new_spanned(&arm.pat, "unsupported by #[sorted]");
                self.errors.push(error);
            }
        }
    }
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
                        let error = syn::Error::new(
                            variant.ident.span(),
                            format!("{} should sort before {}", ident, var_str_vec[should_be]),
                        );
                        return Err(error);
                    }
                }
                var_str_vec.push(ident);
            }
        }
        Ok(())
    };

    check_variants_order(&input)
}
