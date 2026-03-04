# Phase 5: Generic Parameter Tracking

## Goal

Automatically detect which generic type parameters and lifetimes are used by which fields, enabling proc macros to emit minimal `where` clauses without over-constraining.

## Problem

When generating trait impls for generic types, developers must manually figure out which type parameters need bounds:

```rust
// Manual approach:
// Does field `data: Vec<T>` use T? Yes → need `T: Clone` in where clause
// Does field `id: u64` use T? No → don't constrain
```

Getting this wrong either over-constrains (preventing valid usage) or under-constrains (compile error in generated code).

## Solution

`#[zyn::input]` structs can opt into generic tracking with a `#[zyn(track_generics)]` flag:

```rust
#[zyn::input]
#[zyn(track_generics)]
struct MyInput {
    ident: syn::Ident,
    generics: syn::Generics,
    fields: Vec<syn::Field>,
}

// After parsing:
let used_params = input.type_params_used_by(&input.fields[0]);
// Returns which of the struct's generic params appear in that field's type
```

## Design

### Analysis

For each field, scan its `syn::Type` for references to the struct's declared type parameters and lifetimes. Build a mapping: `field → Set<TypeParam>`.

### API

The `#[zyn::input]` macro generates an additional method:

```rust
impl MyInput {
    pub fn type_params_for(&self, field: &syn::Field) -> Vec<&syn::TypeParam> {
        // returns only the generic params that appear in this field's type
    }

    pub fn lifetimes_for(&self, field: &syn::Field) -> Vec<&syn::Lifetime> {
        // returns only the lifetimes that appear in this field's type
    }
}
```

### Template Usage

```rust
zyn! {
    impl<{{ generics }}> MyTrait for {{ ident }}<{{ generics }}>
    where
        @for (field in fields.iter()) {
            @for (param in input.type_params_for(field)) {
                {{ param }}: Clone,
            }
        }
    {
        // ...
    }
}
```

## Files to Modify

| File | Change |
|---|---|
| `crates/core/src/input/generics.rs` | **New** — type param / lifetime scanning logic |
| `crates/derive/src/input.rs` | Generate `type_params_for` / `lifetimes_for` when `track_generics` is set |

## Tests

- Field `data: Vec<T>` with generic `T` → `T` is tracked
- Field `id: u64` → no params tracked
- Field `pair: (T, U)` → both `T` and `U` tracked
- Lifetime `'a` in `&'a str` → lifetime tracked
- Nested generic `HashMap<K, Vec<V>>` → both `K` and `V` tracked
