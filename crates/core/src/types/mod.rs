mod derive;
mod item;

use quote::ToTokens;

/// The proc macro input context. Wraps either a `DeriveInput` or an `Item`.
#[derive(Clone)]
pub enum Input {
    Derive(syn::DeriveInput),
    Item(syn::Item),
}

impl Input {
    pub fn is_derive(&self) -> bool {
        matches!(self, Self::Derive(_))
    }

    pub fn is_item(&self) -> bool {
        matches!(self, Self::Item(_))
    }
}

impl Input {
    pub fn as_derive(&self) -> Option<&syn::DeriveInput> {
        match self {
            Self::Derive(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_item(&self) -> Option<&syn::Item> {
        match self {
            Self::Item(i) => Some(i),
            _ => None,
        }
    }
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => &d.attrs,
            Self::Item(i) => item::attrs(i),
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Derive(d) => &d.ident,
            Self::Item(i) => item::ident(i),
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Derive(d) => &d.generics,
            Self::Item(i) => item::generics(i),
        }
    }

    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Derive(d) => &d.vis,
            Self::Item(i) => item::vis(i),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Derive(syn::parse_str("struct __ZynDefault;").unwrap())
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Derive(d) => d.to_tokens(tokens),
            Self::Item(i) => i.to_tokens(tokens),
        }
    }
}

impl syn::parse::Parse for Input {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        use syn::parse::discouraged::Speculative;

        let fork = input.fork();
        if let Ok(d) = fork.parse::<syn::DeriveInput>() {
            input.advance_to(&fork);
            return Ok(Self::Derive(d));
        }

        Ok(Self::Item(input.parse()?))
    }
}

impl From<syn::DeriveInput> for Input {
    fn from(v: syn::DeriveInput) -> Self {
        Self::Derive(v)
    }
}

impl From<syn::Item> for Input {
    fn from(v: syn::Item) -> Self {
        Self::Item(v)
    }
}
