mod element_node;
mod for_node;
mod group_node;
mod if_node;
mod interp_node;
mod match_node;
mod pipe_node;
mod throw_node;
mod tokens_node;

pub use element_node::ElementNode;
pub use for_node::ForNode;
pub use group_node::GroupNode;
pub use if_node::IfNode;
pub use interp_node::InterpNode;
pub use match_node::MatchNode;
pub use pipe_node::PipeNode;
pub use throw_node::ThrowNode;
pub use tokens_node::TokensNode;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use quote::ToTokens;

use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

pub enum Node {
    Tokens(TokensNode),
    Interp(InterpNode),
    If(IfNode),
    For(ForNode),
    Match(MatchNode),
    Group(GroupNode),
    Throw(ThrowNode),
    Element(ElementNode),
}

impl Node {
    pub fn is_tokens(&self) -> bool {
        matches!(self, Self::Tokens(_))
    }

    pub fn is_interp(&self) -> bool {
        matches!(self, Self::Interp(_))
    }

    pub fn is_if(&self) -> bool {
        matches!(self, Self::If(_))
    }

    pub fn is_for(&self) -> bool {
        matches!(self, Self::For(_))
    }

    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_throw(&self) -> bool {
        matches!(self, Self::Throw(_))
    }

    pub fn is_element(&self) -> bool {
        matches!(self, Self::Element(_))
    }
}

impl Node {
    pub fn as_tokens(&self) -> &TokensNode {
        match self {
            Self::Tokens(v) => v,
            _ => panic!("called as_tokens on non-Tokens node"),
        }
    }

    pub fn as_interp(&self) -> &InterpNode {
        match self {
            Self::Interp(v) => v,
            _ => panic!("called as_interp on non-Interp node"),
        }
    }

    pub fn as_if(&self) -> &IfNode {
        match self {
            Self::If(v) => v,
            _ => panic!("called as_if on non-If node"),
        }
    }

    pub fn as_for(&self) -> &ForNode {
        match self {
            Self::For(v) => v,
            _ => panic!("called as_for on non-For node"),
        }
    }

    pub fn as_match(&self) -> &MatchNode {
        match self {
            Self::Match(v) => v,
            _ => panic!("called as_match on non-Match node"),
        }
    }

    pub fn as_group(&self) -> &GroupNode {
        match self {
            Self::Group(v) => v,
            _ => panic!("called as_group on non-Group node"),
        }
    }

    pub fn as_throw(&self) -> &ThrowNode {
        match self {
            Self::Throw(v) => v,
            _ => panic!("called as_throw on non-Throw node"),
        }
    }

    pub fn as_element(&self) -> &ElementNode {
        match self {
            Self::Element(v) => v,
            _ => panic!("called as_element on non-Element node"),
        }
    }
}

impl Node {
    pub fn span(&self) -> Span {
        match self {
            Self::Tokens(v) => v.span(),
            Self::Interp(v) => v.span(),
            Self::If(v) => v.span(),
            Self::For(v) => v.span(),
            Self::Match(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Throw(v) => v.span(),
            Self::Element(v) => v.span(),
        }
    }
}

impl From<TokensNode> for Node {
    fn from(v: TokensNode) -> Self {
        Self::Tokens(v)
    }
}

impl From<InterpNode> for Node {
    fn from(v: InterpNode) -> Self {
        Self::Interp(v)
    }
}

impl From<IfNode> for Node {
    fn from(v: IfNode) -> Self {
        Self::If(v)
    }
}

impl From<ForNode> for Node {
    fn from(v: ForNode) -> Self {
        Self::For(v)
    }
}

impl From<MatchNode> for Node {
    fn from(v: MatchNode) -> Self {
        Self::Match(v)
    }
}

impl From<GroupNode> for Node {
    fn from(v: GroupNode) -> Self {
        Self::Group(v)
    }
}

impl From<ThrowNode> for Node {
    fn from(v: ThrowNode) -> Self {
        Self::Throw(v)
    }
}

impl From<ElementNode> for Node {
    fn from(v: ElementNode) -> Self {
        Self::Element(v)
    }
}

pub struct Element {
    pub nodes: Vec<Node>,
}

impl Element {
    pub fn span(&self) -> Span {
        self.nodes
            .first()
            .map(|n| n.span())
            .unwrap_or_else(Span::call_site)
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut nodes = Vec::new();
        let mut pending = TokenStream::new();

        while !input.is_empty() {
            if input.peek(Token![@]) {
                flush(&mut pending, &mut nodes);
                nodes.push(parse_at(input)?);
            } else if input.peek(syn::token::Brace) {
                flush(&mut pending, &mut nodes);
                nodes.push(parse_brace(input)?);
            } else if input.peek(syn::token::Paren) || input.peek(syn::token::Bracket) {
                flush(&mut pending, &mut nodes);
                nodes.push(input.parse::<GroupNode>()?.into());
            } else {
                let tt: TokenTree = input.parse()?;
                tt.to_tokens(&mut pending);
            }
        }

        flush(&mut pending, &mut nodes);
        Ok(Self { nodes })
    }
}

fn flush(pending: &mut TokenStream, nodes: &mut Vec<Node>) {
    if pending.is_empty() {
        return;
    }

    let span = pending
        .clone()
        .into_iter()
        .next()
        .map(|tt| tt.span())
        .unwrap_or_else(Span::call_site);

    nodes.push(
        TokensNode {
            span,
            stream: pending.clone(),
        }
        .into(),
    );

    *pending = TokenStream::new();
}

fn parse_at(input: ParseStream) -> syn::Result<Node> {
    let at_span = input.parse::<Token![@]>()?.span;
    let ident: syn::Ident = input.parse()?;
    let name = ident.to_string();

    match name.as_str() {
        "if" => {
            let mut v = input.parse::<IfNode>()?;
            v.span = at_span;
            Ok(v.into())
        }
        "for" => {
            let mut v = input.parse::<ForNode>()?;
            v.span = at_span;
            Ok(v.into())
        }
        "match" => {
            let mut v = input.parse::<MatchNode>()?;
            v.span = at_span;
            Ok(v.into())
        }
        "throw" => {
            let mut v = input.parse::<ThrowNode>()?;
            v.span = at_span;
            Ok(v.into())
        }
        "else" => Err(syn::Error::new(at_span, "unexpected @else without @if")),
        _ => {
            let mut v = ElementNode::parse_with_ident(input, ident)?;
            v.span = at_span;
            Ok(v.into())
        }
    }
}

fn parse_brace(input: ParseStream) -> syn::Result<Node> {
    let content;
    let brace = syn::braced!(content in input);
    let span = brace.span.join();

    let fork = content.fork();
    let is_interp = if fork.peek(syn::token::Brace) {
        let inner;
        syn::braced!(inner in fork);
        fork.is_empty()
    } else {
        false
    };

    if is_interp {
        let inner;
        syn::braced!(inner in content);

        let mut expr = TokenStream::new();
        let mut pipes = Vec::new();

        while !inner.is_empty() {
            if inner.peek(Token![|]) {
                inner.parse::<Token![|]>()?;
                pipes.push(inner.parse::<PipeNode>()?);
            } else if pipes.is_empty() {
                let tt: TokenTree = inner.parse()?;
                tt.to_tokens(&mut expr);
            } else {
                break;
            }
        }

        if expr.is_empty() {
            return Err(syn::Error::new(span, "empty interpolation"));
        }

        Ok(InterpNode { span, expr, pipes }.into())
    } else {
        let body = content.parse::<Element>()?;
        Ok(GroupNode {
            span,
            delimiter: proc_macro2::Delimiter::Brace,
            body: Box::new(body),
        }
        .into())
    }
}
