mod emit;
mod structs;

use zyn_core::proc_macro2::TokenStream;
use zyn_core::syn;

pub fn expand(input: TokenStream) -> TokenStream {
    match syn::parse2::<syn::DeriveInput>(input) {
        Ok(input) => match &input.data {
            syn::Data::Struct(_) => structs::expand(input),
            syn::Data::Enum(_) => syn::Error::new(
                syn::spanned::Spanned::span(&input.ident),
                "enums are not supported yet; use Phase 3",
            )
            .to_compile_error(),
            syn::Data::Union(_) => syn::Error::new(
                syn::spanned::Spanned::span(&input.ident),
                "unions are not supported by #[derive(Attribute)]",
            )
            .to_compile_error(),
        },
        Err(e) => e.to_compile_error(),
    }
}
