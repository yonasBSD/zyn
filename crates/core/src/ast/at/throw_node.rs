use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use proc_macro2_diagnostics::Diagnostic;
use proc_macro2_diagnostics::Level;

use syn::parse::Parse;
use syn::parse::ParseStream;

use super::diag_child_node::DiagChildNode;

use crate::Expand;

pub struct ThrowNode {
    pub span: Span,
    pub message: syn::LitStr,
    pub children: Vec<DiagChildNode>,
}

impl ThrowNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for ThrowNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message: syn::LitStr = input.parse()?;

        let children = if input.peek(syn::token::Brace) {
            let body;
            syn::braced!(body in input);
            DiagChildNode::parse_many(&body)?
        } else {
            Vec::new()
        };

        Ok(Self {
            span: message.span(),
            message,
            children,
        })
    }
}

impl Expand for ThrowNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let mut diag = Diagnostic::spanned(self.span, Level::Error, self.message.value());

        for child in &self.children {
            diag = child.attach_to(diag);
        }

        diag.emit_as_item_tokens()
    }
}
