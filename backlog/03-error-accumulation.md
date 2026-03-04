# Phase 3: Error Accumulation

## Goal

When `#[zyn::input]` parsing encounters multiple errors (missing fields, wrong types, unknown attributes), all errors are collected and emitted together instead of failing on the first one.

## Problem

Today, `Args::parse` and manual attribute extraction use `?` to short-circuit on the first error. Users see one error, fix it, recompile, see the next error, fix it — frustrating feedback loop.

## Solution

The generated `Parse` impl from `#[zyn::input]` uses an internal error accumulator. All field extractions are attempted, errors are collected, and the complete set is emitted at the end.

```bash
error: missing required field `name` in #[my_derive(...)]
  --> src/lib.rs:5:1
   |
error: unknown attribute key `naem` in #[my_derive(...)]
  --> src/lib.rs:5:20
   |
   = help: did you mean `name`?
```

## Design

### Accumulator

A simple `Vec<Diagnostic>` collected during extraction:

```rust
let mut errors: Vec<Diagnostic> = Vec::new();

let skip = __zyn_args.has("skip");
let rename_to = match __zyn_args.get("name") {
    Some(arg) => Some(/* extract */),
    None => None,
};
let format = match __zyn_args.get("format") {
    Some(arg) => /* extract */,
    None => {
        errors.push(Diagnostic::spanned(span, Level::Error, "missing required field `format`"));
        Default::default() // placeholder to continue collecting
    }
};

if !errors.is_empty() {
    // emit all errors as a combined diagnostic
    return Err(/* combined error */);
}
```

### Unknown Attribute Detection

After extracting all known fields, any remaining keys in `Args` that weren't consumed are flagged as unknown:

```rust
for arg in __zyn_args.iter() {
    if let Some(name) = arg.name() {
        if !known_fields.contains(&name.to_string()) {
            errors.push(Diagnostic::spanned(
                name.span(),
                Level::Error,
                format!("unknown attribute key `{}`", name),
            ));
        }
    }
}
```

### "Did you mean?" Suggestions

For unknown keys, compute edit distance against known field names and suggest close matches via `@help`:

```rust
if let Some(suggestion) = find_closest(name_str, &known_fields) {
    diag = diag.span_help(name.span(), format!("did you mean `{}`?", suggestion));
}
```

## Files to Modify

| File | Change |
|---|---|
| `crates/derive/src/input.rs` | Add error accumulation to generated Parse impl |
| `crates/core/src/meta/args.rs` | Optional: add `consume`/`remaining` methods for tracking used keys |

## Tests

- Two missing required fields → both errors shown
- Unknown attribute key → error with help suggestion
- Mix of valid + invalid fields → valid ones still parse, all errors shown
- Zero errors → normal success path unchanged
