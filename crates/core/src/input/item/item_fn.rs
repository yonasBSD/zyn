use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemFn(pub syn::ItemFn);

impl Parse for ItemFn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemFn {
    type Target = syn::ItemFn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemFn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemFn {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemFn {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Fn(v)) => Ok(v.clone()),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected fn item input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let f: ItemFn = syn::parse_str("fn add(a: u32, b: u32) -> u32 { a + b }").unwrap();
        assert_eq!(f.sig.ident.to_string(), "add");
    }
}
