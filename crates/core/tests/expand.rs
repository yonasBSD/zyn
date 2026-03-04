use quote::quote;
use zyn_core::ast::Element;

mod tokens {
    use super::*;

    #[test]
    fn plain() {
        let element: Element = syn::parse_str("struct Foo ;").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::zyn::proc_macro2::TokenStream::new();
                __zyn_ts_0.extend(::zyn::quote::quote!(struct Foo ;));
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod interp {
    use super::*;

    #[test]
    fn no_pipes() {
        let element: Element = syn::parse_str("{{ name }}").unwrap();
        let result = element.to_token_stream();
        let expected = quote! {
            {
                let mut __zyn_ts_0 = ::zyn::proc_macro2::TokenStream::new();
                ::zyn::quote::ToTokens::to_tokens(&(name), &mut __zyn_ts_0);
                __zyn_ts_0
            }
        };
        assert_eq!(result.to_string(), expected.to_string());
    }
}

mod throw {
    use super::*;

    #[test]
    fn generates_compile_error() {
        let element: Element = syn::parse_str("@throw \"bad input\"").unwrap();
        let result = element.to_token_stream().to_string();
        assert!(
            result.contains("compile_error"),
            "expected compile_error!, got: {result}"
        );
        assert!(
            result.contains("bad input"),
            "expected message, got: {result}"
        );
    }
}

mod warn {
    use super::*;

    #[test]
    fn generates_warning() {
        let element: Element = syn::parse_str("@warn \"use new_api instead\"").unwrap();
        let result = element.to_token_stream().to_string();
        assert!(!result.is_empty(), "expected non-empty warning output");
        assert!(
            result.contains("use new_api instead"),
            "expected warning message, got: {result}"
        );
    }

    #[test]
    fn with_note_child() {
        let element: Element =
            syn::parse_str("@warn \"deprecated\" { @note \"see migration guide\" }").unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("deprecated"), "expected warning message");
        assert!(
            result.contains("see migration guide"),
            "expected note in message"
        );
    }

    #[test]
    fn with_help_child() {
        let element: Element =
            syn::parse_str("@warn \"deprecated\" { @help \"use new_api() instead\" }").unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("deprecated"), "expected warning message");
        assert!(
            result.contains("use new_api() instead"),
            "expected help in message"
        );
    }

    #[test]
    fn with_note_and_help_children() {
        let element: Element = syn::parse_str(
            "@warn \"deprecated\" { @note \"removed in v3\" @help \"migrate to new_api()\" }",
        )
        .unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("deprecated"), "expected warning message");
        assert!(result.contains("removed in v3"), "expected note");
        assert!(result.contains("migrate to new_api()"), "expected help");
    }
}

mod note {
    use super::*;

    #[test]
    fn parses_successfully() {
        let result = syn::parse_str::<Element>("@note \"additional context\"");
        assert!(result.is_ok(), "@note should parse: {:?}", result.err());
    }

    #[test]
    fn expands_to_tokens() {
        let element: Element = syn::parse_str("@note \"additional context\"").unwrap();
        let ts = element.to_token_stream().to_string();
        assert!(
            ts.contains("additional context"),
            "expected note message in output, got: {ts}"
        );
    }
}

mod help {
    use super::*;

    #[test]
    fn parses_successfully() {
        let result = syn::parse_str::<Element>("@help \"try this instead\"");
        assert!(result.is_ok(), "@help should parse: {:?}", result.err());
    }

    #[test]
    fn expands_to_tokens() {
        let element: Element = syn::parse_str("@help \"try this instead\"").unwrap();
        let ts = element.to_token_stream().to_string();
        assert!(
            ts.contains("try this instead"),
            "expected help message in output, got: {ts}"
        );
    }
}

mod throw_nested {
    use super::*;

    #[test]
    fn with_note_and_help() {
        let element: Element = syn::parse_str(
            "@throw \"invalid name\" { @note \"must be lowercase\" @help \"try `foo_bar`\" }",
        )
        .unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("compile_error"), "expected compile_error");
        assert!(result.contains("invalid name"), "expected primary message");
        assert!(result.contains("must be lowercase"), "expected note text");
        assert!(result.contains("try `foo_bar`"), "expected help text");
    }

    #[test]
    fn with_note_only() {
        let element: Element =
            syn::parse_str("@throw \"bad value\" { @note \"expected a positive integer\" }")
                .unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("compile_error"), "expected compile_error");
        assert!(result.contains("bad value"), "expected primary message");
        assert!(
            result.contains("expected a positive integer"),
            "expected note text"
        );
    }

    #[test]
    fn with_help_only() {
        let element: Element =
            syn::parse_str("@throw \"missing field\" { @help \"add a `name` field\" }").unwrap();
        let result = element.to_token_stream().to_string();
        assert!(result.contains("compile_error"), "expected compile_error");
        assert!(result.contains("missing field"), "expected primary message");
        assert!(result.contains("add a `name` field"), "expected help text");
    }
}

mod syntax_errors {
    use zyn_core::ast::Element;

    fn parse_err(input: &str) -> String {
        match syn::parse_str::<Element>(input) {
            Err(e) => e.to_string(),
            Ok(_) => panic!("expected parse error for: {input}"),
        }
    }

    #[test]
    fn empty_interpolation() {
        let msg = parse_err("{{ }}");
        assert!(
            msg.contains("empty interpolation"),
            "expected 'empty interpolation', got: {msg}"
        );
    }

    #[test]
    fn throw_missing_message() {
        let msg = parse_err("@throw");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn else_without_if() {
        let msg = parse_err("@else { foo }");
        assert!(
            msg.contains("unexpected @else without @if"),
            "expected 'unexpected @else without @if', got: {msg}"
        );
    }

    #[test]
    fn for_wrong_in_keyword() {
        let msg = parse_err("@for (item from items) { }");
        assert!(
            msg.contains("expected `in`"),
            "expected 'expected `in`', got: {msg}"
        );
    }

    #[test]
    fn element_no_parens() {
        let result = syn::parse_str::<Element>("@my_element");
        assert!(result.is_ok(), "@element without parens should succeed");
    }

    #[test]
    fn element_empty_parens() {
        let result = syn::parse_str::<Element>("@my_element()");
        assert!(result.is_ok(), "@element with empty parens should succeed");
    }

    #[test]
    fn throw_non_string_message() {
        let msg = parse_err("@throw 42");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn warn_missing_message() {
        let msg = parse_err("@warn");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn note_missing_message() {
        let msg = parse_err("@note");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn help_missing_message() {
        let msg = parse_err("@help");
        assert!(
            msg.contains("expected string literal"),
            "expected 'expected string literal', got: {msg}"
        );
    }

    #[test]
    fn invalid_child_in_throw_body() {
        let msg = parse_err("@throw \"msg\" { @if (x) { } }");
        assert!(
            msg.contains("expected `note` or `help`"),
            "expected child directive error, got: {msg}"
        );
    }
}
