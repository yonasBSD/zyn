use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use proc_macro2_diagnostics::Diagnostic;
use proc_macro2_diagnostics::Level;

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;

pub struct HelpNode {
    pub span: Span,
    pub message: syn::LitStr,
}

impl HelpNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for HelpNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let message: syn::LitStr = input.parse()?;

        Ok(Self {
            span: message.span(),
            message,
        })
    }
}

impl Expand for HelpNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        Diagnostic::spanned(self.span, Level::Help, self.message.value()).emit_as_item_tokens()
    }
}
