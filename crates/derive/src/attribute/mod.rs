mod emit;
mod enums;
mod structs;

use zyn_core::__private::proc_macro2::TokenStream;
use zyn_core::syn;

pub fn expand(input: TokenStream) -> TokenStream {
    match syn::parse2::<syn::DeriveInput>(input) {
        Ok(input) => match &input.data {
            syn::Data::Struct(_) => structs::expand(input),
            syn::Data::Enum(_) => enums::expand(input),
            syn::Data::Union(_) => syn::Error::new(
                syn::spanned::Spanned::span(&input.ident),
                "unions are not supported by #[derive(Attribute)]",
            )
            .to_compile_error(),
        },
        Err(e) => e.to_compile_error(),
    }
}
