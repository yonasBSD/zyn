use proc_macro2::Delimiter;
use proc_macro2::Span;

use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Element;

pub struct GroupNode {
    pub span: Span,
    pub delimiter: Delimiter,
    pub body: Box<Element>,
}

impl GroupNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for GroupNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            let paren = syn::parenthesized!(content in input);
            let body = content.parse::<Element>()?;

            Ok(Self {
                span: paren.span.join(),
                delimiter: Delimiter::Parenthesis,
                body: Box::new(body),
            })
        } else if input.peek(syn::token::Bracket) {
            let content;
            let bracket = syn::bracketed!(content in input);
            let body = content.parse::<Element>()?;

            Ok(Self {
                span: bracket.span.join(),
                delimiter: Delimiter::Bracket,
                body: Box::new(body),
            })
        } else if input.peek(syn::token::Brace) {
            let content;
            let brace = syn::braced!(content in input);
            let body = content.parse::<Element>()?;

            Ok(Self {
                span: brace.span.join(),
                delimiter: Delimiter::Brace,
                body: Box::new(body),
            })
        } else {
            Err(input.error("expected a delimited group"))
        }
    }
}
