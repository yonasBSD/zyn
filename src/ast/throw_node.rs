use proc_macro2::Span;
use proc_macro2::TokenStream;

use syn::parse::Parse;
use syn::parse::ParseStream;

pub struct ThrowNode {
    pub span: Span,
    pub message: TokenStream,
}

impl ThrowNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ThrowNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let msg_content;
        syn::parenthesized!(msg_content in input);
        let message: TokenStream = msg_content.parse()?;

        Ok(Self {
            span: Span::call_site(),
            message,
        })
    }
}
