# Phase 2: Extract shared utilities from element.rs

Extract `has_input_attr` and diagnostic macro generation from `element.rs` into shared modules for reuse by `derive_macro.rs` and `attribute_macro.rs`.

## Changes

**Modified:** `crates/derive/src/element.rs`
- Move `has_input_attr` (line 26) to a shared location (e.g. `pub(crate)` in element.rs, or a new `crates/derive/src/shared.rs`)
- Extract diagnostic macro generation (lines 120-190) into a reusable function returning `TokenStream`

## Verify
```
cargo test --workspace
```
