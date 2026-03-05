pub mod derive;
pub mod item;

pub use derive::*;
pub use item::*;

pub enum Input {
    Derive(DeriveInput),
    Item(ItemInput),
}

impl Input {
    pub fn attrs(&self) -> &[syn::Attribute] {
        match self {
            Self::Derive(d) => d.attrs(),
            Self::Item(i) => i.attrs(),
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        match self {
            Self::Derive(d) => d.ident(),
            Self::Item(i) => i.ident(),
        }
    }

    pub fn generics(&self) -> &syn::Generics {
        match self {
            Self::Derive(d) => d.generics(),
            Self::Item(i) => i.generics(),
        }
    }

    pub fn vis(&self) -> &syn::Visibility {
        match self {
            Self::Derive(d) => d.vis(),
            Self::Item(i) => i.vis(),
        }
    }
}

impl Default for Input {
    fn default() -> Self {
        Self::Derive(syn::parse_str("struct __ZynDefault;").unwrap())
    }
}

impl quote::ToTokens for Input {
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

        if let Ok(d) = fork.parse::<DeriveInput>() {
            input.advance_to(&fork);
            return Ok(Self::Derive(d));
        }

        Ok(Self::Item(input.parse()?))
    }
}

impl From<DeriveInput> for Input {
    fn from(v: DeriveInput) -> Self {
        Self::Derive(v)
    }
}

impl From<ItemInput> for Input {
    fn from(v: ItemInput) -> Self {
        Self::Item(v)
    }
}
