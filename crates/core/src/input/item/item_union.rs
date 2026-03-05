use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemUnion(pub syn::ItemUnion);

impl Parse for ItemUnion {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemUnion {
    type Target = syn::ItemUnion;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemUnion {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemUnion {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemUnion {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Union(v)) => Ok(v.clone()),
            crate::input::Input::Derive(crate::input::DeriveInput::Union(u)) => {
                Ok(ItemUnion(syn::ItemUnion {
                    attrs: u.attrs.clone(),
                    vis: u.vis.clone(),
                    union_token: syn::Token![union](proc_macro2::Span::call_site()),
                    ident: u.ident.clone(),
                    generics: u.generics.clone(),
                    fields: u.data.fields.clone(),
                }))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected union input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let u: ItemUnion = syn::parse_str("union Bits { i: i32, f: f32 }").unwrap();
        assert_eq!(u.ident.to_string(), "Bits");
    }
}
