use syn::punctuated::Punctuated;
use syn::spanned::Spanned;

use crate::diagnostic::Diagnostics;
use crate::types::Input;

use super::FromInput;

/// Element extractor that pulls enum variants from the input.
///
/// Errors at compile time if the input is not an enum. Access the inner
/// `Punctuated<syn::Variant, syn::token::Comma>` via `Deref` or the `inner()` method.
///
/// ```ignore
/// #[zyn::element]
/// fn my_element(#[zyn(input)] variants: zyn::Variants) -> proc_macro2::TokenStream {
///     // variants.iter() — accessed via Deref to Punctuated<syn::Variant, syn::token::Comma>
/// }
/// ```
pub struct Variants(Punctuated<syn::Variant, syn::token::Comma>);

impl Variants {
    /// Consumes the wrapper and returns the inner `Punctuated<syn::Variant, syn::token::Comma>`.
    pub fn inner(self) -> Punctuated<syn::Variant, syn::token::Comma> {
        self.0
    }
}

impl std::ops::Deref for Variants {
    type Target = Punctuated<syn::Variant, syn::token::Comma>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Variants {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromInput for Variants {
    fn from_input(input: &Input) -> crate::Result<Self> {
        match input {
            Input::Derive(d) => match &d.data {
                syn::Data::Enum(e) => Ok(Variants(e.variants.clone())),
                _ => Err(Diagnostics::error(
                    d.ident.span(),
                    "expected enum input for Variants extractor",
                )),
            },
            Input::Item(syn::Item::Enum(e)) => Ok(Variants(e.variants.clone())),
            _ => Err(Diagnostics::error(
                input.span(),
                "expected enum input for Variants extractor",
            )),
        }
    }
}
