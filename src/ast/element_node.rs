use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::Element;

pub struct ElementNode {
    pub span: Span,
    pub name: TokenStream,
    pub props: Vec<(syn::Ident, TokenStream)>,
    pub children: Option<Box<Element>>,
}

impl ElementNode {
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn parse_with_ident(input: ParseStream, first_ident: syn::Ident) -> syn::Result<Self> {
        let span = first_ident.span();

        let mut name = TokenStream::new();
        first_ident.to_tokens(&mut name);

        while input.peek(Token![::]) {
            let colons: Token![::] = input.parse()?;
            colons.to_tokens(&mut name);
            let segment: syn::Ident = input.parse()?;
            segment.to_tokens(&mut name);
        }

        let props_content;
        syn::braced!(props_content in input);

        let mut props = Vec::new();

        while !props_content.is_empty() {
            let prop_name: syn::Ident = props_content.parse()?;
            props_content.parse::<Token![:]>()?;

            let mut value = TokenStream::new();

            while !props_content.is_empty() && !props_content.peek(Token![,]) {
                let tt: TokenTree = props_content.parse()?;
                tt.to_tokens(&mut value);
            }

            props.push((prop_name, value));

            if props_content.peek(Token![,]) {
                props_content.parse::<Token![,]>()?;
            }
        }

        let children = if input.peek(syn::token::Brace) {
            let children_content;
            syn::braced!(children_content in input);
            Some(Box::new(children_content.parse::<Element>()?))
        } else {
            None
        };

        Ok(Self {
            span,
            name,
            props,
            children,
        })
    }
}

impl Parse for ElementNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let first_ident: syn::Ident = input.parse()?;
        let span = first_ident.span();

        let mut name = TokenStream::new();
        first_ident.to_tokens(&mut name);

        while input.peek(Token![::]) {
            let colons: Token![::] = input.parse()?;
            colons.to_tokens(&mut name);
            let segment: syn::Ident = input.parse()?;
            segment.to_tokens(&mut name);
        }

        let props_content;
        syn::braced!(props_content in input);

        let mut props = Vec::new();

        while !props_content.is_empty() {
            let prop_name: syn::Ident = props_content.parse()?;
            props_content.parse::<Token![:]>()?;

            let mut value = TokenStream::new();

            while !props_content.is_empty() && !props_content.peek(Token![,]) {
                let tt: TokenTree = props_content.parse()?;
                tt.to_tokens(&mut value);
            }

            props.push((prop_name, value));

            if props_content.peek(Token![,]) {
                props_content.parse::<Token![,]>()?;
            }
        }

        let children = if input.peek(syn::token::Brace) {
            let children_content;
            syn::braced!(children_content in input);
            Some(Box::new(children_content.parse::<Element>()?))
        } else {
            None
        };

        Ok(Self {
            span,
            name,
            props,
            children,
        })
    }
}
