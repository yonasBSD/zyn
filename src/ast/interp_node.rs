use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::PipeNode;

pub struct InterpNode {
    pub span: Span,
    pub expr: TokenStream,
    pub pipes: Vec<PipeNode>,
}

impl InterpNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for InterpNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let brace = syn::braced!(content in input);
        let span = brace.span.join();

        let inner;
        syn::braced!(inner in content);

        let mut expr = TokenStream::new();
        let mut pipes = Vec::new();

        while !inner.is_empty() {
            if inner.peek(Token![|]) {
                inner.parse::<Token![|]>()?;
                pipes.push(inner.parse::<PipeNode>()?);
            } else if pipes.is_empty() {
                let tt: TokenTree = inner.parse()?;
                tt.to_tokens(&mut expr);
            } else {
                break;
            }
        }

        if expr.is_empty() {
            return Err(syn::Error::new(span, "empty interpolation"));
        }

        Ok(Self { span, expr, pipes })
    }
}
