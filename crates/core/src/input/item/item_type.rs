use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemType(pub syn::ItemType);

impl Parse for ItemType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemType {
    type Target = syn::ItemType;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemType {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemType {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Type(v)) => Ok(v.clone()),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected type item input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let t: ItemType = syn::parse_str("type Meters = f64;").unwrap();
        assert_eq!(t.ident.to_string(), "Meters");
    }
}
