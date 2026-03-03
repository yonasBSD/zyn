# Phase 2: Parser ✅

## Status: Merged into Phase 1

Parsing logic was merged into the AST module (`ast/`). Each node type implements `syn::parse::Parse` in its own file. The root parse logic (`Parse for Element`) lives in `ast/mod.rs`. There is no separate `parse.rs`.

See [1-ast.md](1-ast.md) for full details.
