# Phase 6: #[element] Attribute Macro ✅

## Status: Complete

Implemented in `crates/derive/src/element.rs`. The `#[zyn::element]` attribute macro transforms a snake_case function into a PascalCase struct + `Render` impl.

- Function name converted from snake_case to PascalCase (e.g. `field_decl` → `FieldDecl`)
- Function parameters become `pub` struct fields
- Function body becomes the `render()` body with `let field = &self.field;` bindings
- Function visibility applied to the struct
- Children supported via a `children: proc_macro2::TokenStream` parameter
- Re-exported from root crate via `pub use zyn_derive::*`

Tests: `tests/zyn.rs` — `elements::basic_element` and `elements::element_with_children`.
