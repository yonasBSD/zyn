pub mod ast;
pub mod case;
pub mod debug;
pub mod diagnostic;
pub mod extract;
pub mod ident;
pub mod input;
pub mod meta;
pub mod pipes;

#[cfg(feature = "ext")]
pub mod ext;

pub use diagnostic::*;
pub use extract::*;
pub use input::*;
pub use meta::*;
pub use pipes::*;

pub use proc_macro2::{Span, TokenStream};
pub use quote::{ToTokens, format_ident};
pub use syn;

#[doc(hidden)]
pub mod __private {
    pub use proc_macro2;
    pub use quote;
}

pub trait Expand {
    fn expand(
        &self,
        output: &proc_macro2::Ident,
        idents: &mut ident::Iter,
    ) -> proc_macro2::TokenStream;
}

pub trait Render {
    fn render(&self, input: &input::Input) -> proc_macro2::TokenStream;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
