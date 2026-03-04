use proc_macro2::Span;
use proc_macro2_diagnostics::Diagnostic;

use syn::Token;
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::parse::ParseStream;

use super::help_node::HelpNode;
use super::note_node::NoteNode;

pub enum DiagChildNode {
    Note(NoteNode),
    Help(HelpNode),
}

impl DiagChildNode {
    pub fn span(&self) -> Span {
        match self {
            Self::Note(v) => v.span(),
            Self::Help(v) => v.span(),
        }
    }

    pub(super) fn parse_many(input: ParseStream) -> syn::Result<Vec<Self>> {
        let mut children = Vec::new();

        while !input.is_empty() {
            children.push(input.parse::<Self>()?);
        }

        Ok(children)
    }

    pub fn attach_to(&self, diag: Diagnostic) -> Diagnostic {
        match self {
            Self::Note(v) => diag.span_note(v.span, v.message.value()),
            Self::Help(v) => diag.span_help(v.span, v.message.value()),
        }
    }
}

impl Parse for DiagChildNode {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![@]>()?;
        let ident: syn::Ident = input.call(syn::Ident::parse_any)?;

        match ident.to_string().as_str() {
            "note" => Ok(Self::Note(input.parse::<NoteNode>()?)),
            "help" => Ok(Self::Help(input.parse::<HelpNode>()?)),
            other => Err(syn::Error::new_spanned(
                ident,
                format!("expected `note` or `help`, found `{}`", other),
            )),
        }
    }
}
