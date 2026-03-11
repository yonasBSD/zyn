//! Test utilities for the zyn proc macro framework.
//!
//! Provides assertion macros for comparing token streams and inspecting
//! diagnostics produced by [`Output`](crate::Output). Since `zyn!` returns
//! `Output`, all macros work directly on template expansion results.
//!
//! # Setting up tests
//!
//! Element integration tests require an `input` variable in scope. Use
//! [`parse!`](crate::parse) to create one:
//!
//! ```ignore
//! fn dummy_input() -> zyn::Input {
//!     zyn::parse!("struct Test;" => zyn::syn::DeriveInput)
//!         .unwrap()
//!         .into()
//! }
//! ```
//!
//! # Comparing token output
//!
//! Use [`assert_tokens!`] to compare the full output of a template against
//! an expected `quote!` expression:
//!
//! ```ignore
//! #[test]
//! fn generates_function() {
//!     let input = dummy_input();
//!     let output = zyn::zyn!(fn {{ name }}() {});
//!     let expected = zyn::quote::quote!(fn hello() {});
//!     zyn::assert_tokens!(output, expected);
//! }
//! ```
//!
//! For partial matching, use [`assert_tokens_contain!`]:
//!
//! ```ignore
//! zyn::assert_tokens_contain!(output, "fn hello");
//! ```
//!
//! To verify empty output (e.g., after `bail!`):
//!
//! ```ignore
//! zyn::assert_tokens_empty!(output);
//! ```
//!
//! # Asserting diagnostics
//!
//! When an element emits errors, warnings, or other diagnostics, they are
//! carried in the [`Output`](crate::Output). Assert on them by level and message:
//!
//! ```ignore
//! #[zyn::element]
//! fn validated(name: zyn::syn::Ident) -> zyn::TokenStream {
//!     if name == "forbidden" {
//!         bail!("reserved identifier `{}`", name);
//!     }
//!     zyn::zyn!(fn {{ name }}() {})
//! }
//!
//! #[test]
//! fn rejects_forbidden_name() {
//!     let input = dummy_input();
//!     let output = zyn::zyn!(@validated(name = zyn::format_ident!("forbidden")));
//!     zyn::assert_diagnostic_error!(output, "reserved identifier");
//!     zyn::assert_tokens_empty!(output);
//! }
//!
//! #[test]
//! fn accepts_valid_name() {
//!     let input = dummy_input();
//!     let output = zyn::zyn!(@validated(name = zyn::format_ident!("hello")));
//!     zyn::assert_tokens_contain!(output, "fn hello");
//! }
//! ```
//!
//! Warnings and notes don't block output — both tokens and diagnostics are present:
//!
//! ```ignore
//! #[zyn::element]
//! fn deprecated_el(name: zyn::syn::Ident) -> zyn::TokenStream {
//!     warn!("this element is deprecated");
//!     zyn::zyn!(fn {{ name }}() {})
//! }
//!
//! #[test]
//! fn warning_does_not_block_output() {
//!     let input = dummy_input();
//!     let output = zyn::zyn!(@deprecated_el(name = zyn::format_ident!("hello")));
//!     zyn::assert_tokens_contain!(output, "fn hello");
//!     zyn::assert_diagnostic_warning!(output, "deprecated");
//! }
//! ```
//!
//! # Macro reference
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | [`assert_tokens!`] | Compare two token streams (raw-formatted diff on failure) |
//! | [`assert_tokens_empty!`] | Assert no tokens produced |
//! | [`assert_tokens_contain!`] | Check for substring in raw token output |
//! | [`assert_diagnostic!`] | Assert diagnostic at a specific level with message |
//! | [`assert_diagnostic_error!`] | Assert error diagnostic with message |
//! | [`assert_diagnostic_warning!`] | Assert warning diagnostic with message |
//! | [`assert_diagnostic_note!`] | Assert note diagnostic with message |
//! | [`assert_diagnostic_help!`] | Assert help diagnostic with message |
//! | [`assert_compile_error!`] | Alias for [`assert_diagnostic_error!`] |
//!
//! With the `pretty` feature:
//!
//! | Macro | Purpose |
//! |-------|---------|
//! | [`assert_tokens_pretty!`] | Compare using `prettyplease`-formatted output |
//! | [`assert_tokens_contain_pretty!`] | Substring check on pretty-printed output |

mod assert_diagnostic;
mod assert_tokens;
mod assert_tokens_contain;
mod assert_tokens_empty;
