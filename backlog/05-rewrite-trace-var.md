# Phase 5: Rewrite trace-var example

Rewrite the trace-var example to use `#[zyn::attribute]` and element-based architecture.

## Changes

**`examples/trace-var/src/lib.rs`:**
- Replace `#[proc_macro_attribute] pub fn trace_var(...)` with `#[zyn::attribute] fn trace_var(...)`
- Refactor folder logic into a `trace_var_fold` element
- Remove manual `parse_input!` and `Input` construction

```rust
#[zyn::element]
fn trace_var_fold(item: zyn::syn::ItemFn, vars: HashSet<Ident>) -> TokenStream2 {
    let mut folder = TraceVarFolderInner { input: &input, vars };
    zyn::zyn!({ { folder.fold_item_fn(item) } })
}

#[zyn::attribute]
fn trace_var(
    #[zyn(input)] item: zyn::syn::ItemFn,
    args: zyn::Args,
) -> TokenStream2 {
    let vars: HashSet<Ident> = args.iter().filter_map(|a| a.name().cloned()).collect();
    zyn::zyn!(@trace_var_fold(item = item, vars = vars))
}
```

**`examples/trace-var/src/folder.rs`:**
- Rename struct to `TraceVarFolderInner`
- Add `input: &'a zyn::Input` field
- Add lifetime parameter
- Fold methods bind `let input = self.input;` for `zyn!` element calls

## Verify
```
cargo test --workspace
```
