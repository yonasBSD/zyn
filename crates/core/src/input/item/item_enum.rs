use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemEnum(pub syn::ItemEnum);

impl Parse for ItemEnum {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemEnum {
    type Target = syn::ItemEnum;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemEnum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemEnum {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Enum(v)) => Ok(v.clone()),
            crate::input::Input::Derive(crate::input::DeriveInput::Enum(e)) => {
                Ok(ItemEnum(syn::ItemEnum {
                    attrs: e.attrs.clone(),
                    vis: e.vis.clone(),
                    enum_token: syn::Token![enum](proc_macro2::Span::call_site()),
                    ident: e.ident.clone(),
                    generics: e.generics.clone(),
                    brace_token: syn::token::Brace::default(),
                    variants: e.data.variants.clone(),
                }))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected enum input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let e: ItemEnum = syn::parse_str("enum Color { Red, Green, Blue }").unwrap();
        assert_eq!(e.ident.to_string(), "Color");
    }
}
