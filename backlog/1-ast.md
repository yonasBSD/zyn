# Phase 1 + 2: AST Types + Parsing + Traits ✅

## Status: Complete

## Structure

```
crates/derive/src/
  lib.rs                  mod ast;
  ast/
    mod.rs                Node enum, Element struct, Parse impls, From impls, is_*/as_*/span()
    tokens_node.rs        Tokens struct
    interp_node.rs        Interp struct + Parse
    if_node.rs            If struct + Parse (with @else if/@else)
    for_node.rs           For struct + Parse
    match_node.rs         Match struct + Parse
    group_node.rs         Group struct + Parse (paren/bracket/brace)
    throw_node.rs         Throw struct + Parse
    element_node.rs       ElementNode struct + Parse + parse_with_ident()
    pipe_node.rs          Pipe struct + Parse

src/
  lib.rs                  Render trait, Pipe trait
```

## Traits (in `src/lib.rs`)

```rust
pub trait Render {
    fn render(&self) -> syn::Result<proc_macro2::TokenStream>;
}

pub trait Pipe {
    type Input;
    type Output: quote::ToTokens;
    fn pipe(&self, input: Self::Input) -> Self::Output;
}
```

## AST (in `crates/derive/src/ast/`)

Each Node variant is a standalone struct in its own `_node.rs` file with:
- `pub` fields including `span: Span`
- `impl Parse` for parsing from a `ParseStream`
- `span(&self) -> Span` method

### Node enum (`mod.rs`)

```rust
pub enum Node {
    Tokens(Tokens),
    Interp(Interp),
    If(If),
    For(For),
    Match(Match),
    Group(Group),
    Throw(Throw),
    Element(ElementNode),
}
```

### Element (root container)

```rust
pub struct Element {
    pub nodes: Vec<Node>,
}
```

`impl Parse for Element` — main loop dispatches to variant parsers based on `@`, `{{ }}`, group delimiters, or passthrough token accumulation.

### From impls

`From<T> for Node` implemented for all variant types: `Tokens`, `Interp`, `If`, `For`, `Match`, `Group`, `Throw`, `ElementNode`.

### Methods on Node

Predicates: `is_tokens`, `is_interp`, `is_if`, `is_for`, `is_match`, `is_group`, `is_throw`, `is_element`

Accessors (panic on wrong variant): `as_tokens`, `as_interp`, `as_if`, `as_for`, `as_match`, `as_group`, `as_throw`, `as_element`

Span: `span(&self) -> Span` (delegates to inner struct)

### Parsing

- `Parse for Element` — root loop, token accumulation, dispatch
- `parse_at` — `@` directive dispatch (if/for/match/throw/else error/element)
- `parse_brace` — `{{ }}` interpolation detection via fork, or recursive brace Group
- Each struct's `Parse` impl handles its own syntax after the keyword
- `ElementNode::parse_with_ident` — for when `@` + ident already consumed
- `is_at_else` — fork-based `@else` lookahead

Note: `syn::spanned::Spanned` is sealed, so we use inherent `span()` methods.
