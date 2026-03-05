use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemTrait(pub syn::ItemTrait);

impl Parse for ItemTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemTrait {
    type Target = syn::ItemTrait;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemTrait {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemTrait {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Trait(v)) => Ok(v.clone()),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected trait item input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let t: ItemTrait = syn::parse_str("trait Greet { fn hello(&self); }").unwrap();
        assert_eq!(t.ident.to_string(), "Greet");
    }
}
