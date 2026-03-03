use proc_macro2::Span;
use proc_macro2::TokenStream;

use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Element;

pub struct ForNode {
    pub span: Span,
    pub binding: syn::Ident,
    pub iter: TokenStream,
    pub body: Box<Element>,
}

impl ForNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ForNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let params;
        syn::parenthesized!(params in input);

        let binding: syn::Ident = params.parse()?;

        let of_kw: syn::Ident = params.parse()?;
        if of_kw != "of" {
            return Err(syn::Error::new_spanned(of_kw, "expected `of`"));
        }

        let iter: TokenStream = params.parse()?;

        let body_content;
        syn::braced!(body_content in input);
        let body = body_content.parse::<Element>()?;

        Ok(Self {
            span: Span::call_site(),
            binding,
            iter,
            body: Box::new(body),
        })
    }
}
