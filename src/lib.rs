pub mod ast;

pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;

    fn pipe(&self, input: Self::Input) -> Self::Output;
}
