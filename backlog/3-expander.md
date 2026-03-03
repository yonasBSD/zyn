# Phase 3: Expander ✅

## Status: Merged into Phase 1

The `Expand` trait and all `impl Expand` are colocated with their AST node types. There is no separate expander module. See [1-ast.md](1-ast.md) for full details.

Key points:
- `Expand` trait defined in `src/lib.rs`
- `ident::Iter` in `src/ident.rs` replaces the old `next_ident` counter
- Each node struct implements `Expand` in its own file
- `Node` and `AtNode` dispatch to inner struct's `Expand` impl
- `Element::to_token_stream()` is the top-level entry point
