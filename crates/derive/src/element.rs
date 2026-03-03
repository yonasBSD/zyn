use proc_macro2::TokenStream;
use quote::quote;
use syn::FnArg;
use syn::ItemFn;
use syn::ReturnType;
use syn::spanned::Spanned;

pub fn expand(input: TokenStream) -> TokenStream {
    match syn::parse2::<ItemFn>(input) {
        Ok(item) => expand_element(item),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_element(item: ItemFn) -> TokenStream {
    let vis = &item.vis;
    let body = &item.block;

    // Validate return type exists
    if matches!(item.sig.output, ReturnType::Default) {
        return syn::Error::new(
            item.sig.ident.span(),
            "element must return syn::Result<proc_macro2::TokenStream>",
        )
        .to_compile_error();
    }

    // Validate at least one parameter
    if item.sig.inputs.is_empty() {
        return syn::Error::new(
            item.sig.ident.span(),
            "element must have at least one parameter",
        )
        .to_compile_error();
    }

    // Convert snake_case function name to PascalCase struct name
    let struct_name = to_pascal_case_ident(&item.sig.ident);

    // Extract field names and types from parameters
    let mut field_names = Vec::new();
    let mut field_types = Vec::new();

    for arg in &item.sig.inputs {
        match arg {
            FnArg::Typed(pat_type) => {
                let ident = match pat_type.pat.as_ref() {
                    syn::Pat::Ident(pat_ident) => &pat_ident.ident,
                    _ => {
                        return syn::Error::new(
                            pat_type.pat.span(),
                            "element parameters must be simple identifiers",
                        )
                        .to_compile_error();
                    }
                };

                field_names.push(ident.clone());
                field_types.push(pat_type.ty.as_ref().clone());
            }
            FnArg::Receiver(r) => {
                return syn::Error::new(r.span(), "element parameters must be typed")
                    .to_compile_error();
            }
        }
    }

    let ret_type = &item.sig.output;

    quote! {
        #vis struct #struct_name {
            #(pub #field_names: #field_types,)*
        }

        impl ::zyn::Render for #struct_name {
            fn render(&self) #ret_type {
                #(let #field_names = &self.#field_names;)*
                #body
            }
        }
    }
}

fn to_pascal_case_ident(ident: &syn::Ident) -> syn::Ident {
    let s = ident.to_string();
    let mut out = String::new();
    let mut capitalize = true;

    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            out.extend(c.to_uppercase());
            capitalize = false;
        } else {
            out.push(c);
        }
    }

    syn::Ident::new(&out, ident.span())
}
