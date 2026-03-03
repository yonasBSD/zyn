use proc_macro2::Delimiter;
use proc_macro2::Ident;
use proc_macro2::TokenStream;

pub struct Element {
    pub nodes: Vec<Node>,
}

pub enum Node {
    Tokens(TokenStream),

    Interpolation {
        expr: TokenStream,
        pipes: Vec<Pipe>,
    },

    If {
        branches: Vec<(TokenStream, Element)>,
        else_body: Option<Box<Element>>,
    },

    For {
        binding: Ident,
        iter: TokenStream,
        body: Box<Element>,
    },

    Match {
        expr: TokenStream,
        arms: Vec<(TokenStream, Element)>,
    },

    Group {
        delimiter: Delimiter,
        body: Box<Element>,
    },

    Throw {
        message: TokenStream,
    },

    Element {
        name: TokenStream,
        props: Vec<(Ident, TokenStream)>,
        children: Option<Box<Element>>,
    },
}

pub struct Pipe {
    pub name: Ident,
    pub args: Vec<TokenStream>,
}
