# Phase 1: Rename Element → Template

Rename `Element` AST type to `Template`. Move from `ast/mod.rs` to `template.rs`. Add `.render(&input)`.

## Changes

**New:** `crates/core/src/template.rs`
- Move `Element` struct here, rename to `Template`
- Move all impls: `Parse`, `Expand`, `to_token_stream()`, `flush()`, `span()`
- Add `render(&self, input: &Input) -> TokenStream` that wraps expansion with input binding:
  ```rust
  pub fn render(&self, input: &Input) -> TokenStream {
      let expanded = self.to_token_stream();
      quote! {
          {
              let input: ::zyn::Input = ::zyn::parse!(#input).unwrap();
              #expanded
          }
      }
  }
  ```

**Modified:** `crates/core/src/lib.rs`
- Add `pub mod template;` and `pub use template::Template;`

**Modified:** `crates/core/src/ast/mod.rs`
- Remove `Element` struct and all its impls
- Keep `Node` and everything else

**Updated imports:**
- `crates/core/src/ast/at/element_node.rs` — `super::super::Element` → `crate::template::Template`
- `crates/core/src/ast/at/for_node.rs` — same
- `crates/core/src/ast/at/if_node.rs` — same
- `crates/core/src/ast/at/match_node.rs` — same
- `crates/core/src/ast/group_node.rs` — `super::Element` → `crate::template::Template`
- `crates/core/src/debug.rs` — `ast::Element` → `template::Template`
- `crates/derive/src/lib.rs` — `zyn_core::ast::Element` → `zyn_core::Template`
- `tests/diagnostics/syntax.rs` — `zyn_core::ast::Element` → `zyn_core::Template`

## Verify
```
cargo test --workspace
```
