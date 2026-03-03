use proc_macro2::Span;
use proc_macro2::TokenStream;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Element;

pub struct IfNode {
    pub span: Span,
    pub branches: Vec<(TokenStream, Element)>,
    pub else_body: Option<Box<Element>>,
}

impl IfNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for IfNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut branches = Vec::new();
        let mut else_body = None;

        let cond_content;
        syn::parenthesized!(cond_content in input);
        let condition: TokenStream = cond_content.parse()?;

        let body_content;
        syn::braced!(body_content in input);
        let body = body_content.parse::<Element>()?;

        branches.push((condition, body));

        while is_at_else(input) {
            input.parse::<Token![@]>()?;
            input.parse::<syn::Ident>()?;

            if input.peek(syn::Ident)
                && input
                    .fork()
                    .parse::<syn::Ident>()
                    .map(|i| i == "if")
                    .unwrap_or(false)
            {
                input.parse::<syn::Ident>()?;

                let cond_content;
                syn::parenthesized!(cond_content in input);
                let condition: TokenStream = cond_content.parse()?;

                let body_content;
                syn::braced!(body_content in input);
                let body = body_content.parse::<Element>()?;

                branches.push((condition, body));
            } else {
                let body_content;
                syn::braced!(body_content in input);
                else_body = Some(Box::new(body_content.parse::<Element>()?));
                break;
            }
        }

        Ok(Self {
            span: Span::call_site(),
            branches,
            else_body,
        })
    }
}

fn is_at_else(input: ParseStream) -> bool {
    if !input.peek(Token![@]) {
        return false;
    }

    let fork = input.fork();
    let _ = fork.parse::<Token![@]>();

    match fork.parse::<syn::Ident>() {
        Ok(ident) => ident == "else",
        Err(_) => false,
    }
}
