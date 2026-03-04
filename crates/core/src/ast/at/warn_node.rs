use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use proc_macro2_diagnostics::Diagnostic;
use proc_macro2_diagnostics::Level;

use syn::parse::Parse;
use syn::parse::ParseStream;

use super::diag_child_node::DiagChildNode;
use super::diag_child_node::parse_children;

use crate::Expand;

pub struct WarnNode {
    pub span: Span,
    pub message: syn::LitStr,
    pub children: Vec<DiagChildNode>,
}

impl WarnNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for WarnNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message: syn::LitStr = input.parse()?;

        let children = if input.peek(syn::token::Brace) {
            let body;
            syn::braced!(body in input);
            parse_children(&body)?
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

impl Expand for WarnNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let mut diag = Diagnostic::spanned(self.span, Level::Warning, self.message.value());

        for child in &self.children {
            diag = child.attach_to(diag);
        }

        diag.emit_as_item_tokens()
    }
}
