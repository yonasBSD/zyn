use proc_macro2::Span;
use proc_macro2::TokenStream;

pub struct TokensNode {
    pub span: Span,
    pub stream: TokenStream,
}

impl TokensNode {
    pub fn span(&self) -> Span {
        self.span
    }
}
