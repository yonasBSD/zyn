use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;

#[derive(Clone, PartialEq, Eq)]
pub struct ItemStruct(pub syn::ItemStruct);

impl Parse for ItemStruct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl std::ops::Deref for ItemStruct {
    type Target = syn::ItemStruct;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ItemStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ToTokens for ItemStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

impl crate::extract::FromInput for ItemStruct {
    type Error = syn::Error;

    fn from_input(input: &crate::input::Input) -> Result<Self, Self::Error> {
        match input {
            crate::input::Input::Item(crate::input::ItemInput::Struct(v)) => Ok(v.clone()),
            crate::input::Input::Derive(crate::input::DeriveInput::Struct(s)) => {
                Ok(ItemStruct(syn::ItemStruct {
                    attrs: s.attrs.clone(),
                    vis: s.vis.clone(),
                    struct_token: syn::Token![struct](proc_macro2::Span::call_site()),
                    ident: s.ident.clone(),
                    generics: s.generics.clone(),
                    fields: s.data.fields.clone(),
                    semi_token: s.data.semi_token,
                }))
            }
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "expected struct input",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let s: ItemStruct = syn::parse_str("struct Foo { x: u32 }").unwrap();
        assert_eq!(s.ident.to_string(), "Foo");
    }
}
