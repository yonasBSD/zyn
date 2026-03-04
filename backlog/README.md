# Backlog

Planned features for zyn, broken into sequential phases. Each phase builds on the previous.

## Phases

| Phase | Name | Status | Description |
|---|---|---|---|
| [1](./01-attribute-trait.md) | Attribute Trait | planned | `#[derive(Attribute)]` for defining attribute argument schemas; type-safe extraction |
| [2](./02-attribute-extraction.md) | Attribute Extraction | planned | Field-level `default`/`skip`/`rename` annotations on `#[derive(Attribute)]` structs |
| [3](./03-error-accumulation.md) | Error Accumulation | planned | Collect and emit all validation errors together |
| [4](./04-rename-all-and-map.md) | Rename All + Map | planned | Bulk case transformation; custom field transforms via pipes |
| [5](./05-generic-tracking.md) | Generic Parameter Tracking | planned | Auto-detect type param / lifetime usage per field |

## What Gets Replaced

| Current module | Replaced by | Phase |
|---|---|---|
| manual `Arg`/`Args` querying | `Attribute` trait + `#[derive(Attribute)]` | 1 |
| `AttrExt`/`AttrsExt` as primary API | `Attribute::from_attr` / `from_attrs` / `from_attrs_merged` | 1–2 |

`Arg`, `Args`, `AttrExt`, `AttrsExt` remain as low-level internals used by `Attribute`, but are no longer the primary user-facing API. The existing `input/` module (`DeriveInput`, `DeriveStruct`, etc.) is kept as-is.
