use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;

use quote::quote;

use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Expand;
use crate::pascal;

pub struct PipeNode {
    pub span: Span,
    pub name: syn::Ident,
}

impl PipeNode {
    pub fn span(&self) -> Span {
        self.span
    }
}

impl Parse for PipeNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: syn::Ident = input.parse()?;
        let span = name.span();
        Ok(Self { span, name })
    }
}

impl Expand for PipeNode {
    fn expand(&self, _output: &Ident, _idents: &mut crate::ident::Iter) -> TokenStream {
        let name = pascal!(self.name => ident);
        quote! { let __zyn_val = ::zyn::Pipe::pipe(&(#name), __zyn_val); }
    }
}
