use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub struct PipeNode {
    pub name: syn::Ident,
    pub args: Vec<TokenStream>,
}

impl Parse for PipeNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let mut args = Vec::new();

        while input.peek(Token![:]) {
            input.parse::<Token![:]>()?;

            let mut arg = TokenStream::new();

            while !input.is_empty() && !input.peek(Token![:]) && !input.peek(Token![|]) {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut arg);
            }

            args.push(arg);
        }

        Ok(Self { name, args })
    }
}
