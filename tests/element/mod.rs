mod namespaced;

use zyn::quote::quote;
use zyn::syn;

#[zyn::element]
fn greeting(name: zyn::proc_macro2::Ident) -> zyn::proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn basic_element() {
    let result = zyn::zyn!(
        @greeting(name = zyn::quote::format_ident!("hello"))
    );
    let expected = quote!(
        fn hello() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn wrapper(
    name: zyn::proc_macro2::Ident,
    children: zyn::proc_macro2::TokenStream,
) -> zyn::proc_macro2::TokenStream {
    zyn::quote::quote!(struct #name { #children })
}

#[test]
fn element_with_children() {
    let result = zyn::zyn!(
        @wrapper(name = zyn::quote::format_ident!("Foo")) {
            x: i32,
        }
    );
    let expected = quote!(
        struct Foo {
            x: i32,
        }
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element("say_hello")]
fn get_greeting(name: zyn::proc_macro2::Ident) -> zyn::proc_macro2::TokenStream {
    zyn::zyn!(fn {{ name }}() {})
}

#[test]
fn custom_name_override() {
    let result = zyn::zyn!(
        @say_hello(name = zyn::quote::format_ident!("world"))
    );
    let expected = quote!(
        fn world() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn divider() -> zyn::proc_macro2::TokenStream {
    zyn::zyn!(
        const DIVIDER: &str = "---";
    )
}

#[test]
fn zero_param_no_parens() {
    let result = zyn::zyn!(@divider);
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn zero_param_with_parens() {
    let result = zyn::zyn!(@divider());
    let expected = quote!(
        const DIVIDER: &str = "---";
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[zyn::element]
fn container(children: zyn::proc_macro2::TokenStream) -> zyn::proc_macro2::TokenStream {
    zyn::quote::quote!(mod container { #children })
}

#[test]
fn children_without_parens() {
    let result = zyn::zyn!(
        @container {
            struct Inner;
        }
    );
    let expected = quote!(
        mod container {
            struct Inner;
        }
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn element_inside_for_loop() {
    let names = vec![
        zyn::quote::format_ident!("foo"),
        zyn::quote::format_ident!("bar"),
    ];
    let result = zyn::zyn!(
        @for (name in names) {
            @greeting(name = name.clone())
        }
    );
    let expected = quote!(
        fn foo() {}
        fn bar() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[derive(zyn::Attribute)]
#[zyn("my_elem_attr")]
struct ElemAttr {
    rename: Option<String>,
}

#[zyn::element]
fn attr_element(
    attr: zyn::Attr<ElemAttr>,
    label: zyn::proc_macro2::Ident,
) -> zyn::proc_macro2::TokenStream {
    let name_str = attr.0.rename.as_deref().unwrap_or("default");
    let combined = zyn::quote::format_ident!("{}_{}", label, name_str);
    zyn::zyn!(fn {{ combined }}() {})
}

#[test]
fn extractor_param_with_matching_attr() {
    let input: zyn::Input =
        syn::parse_str("#[my_elem_attr(rename = \"custom\")] struct Foo;").unwrap();
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::quote::format_ident!("hello"),
        },
        &input,
    );
    let expected = quote!(
        fn hello_custom() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}

#[test]
fn extractor_absent_uses_default() {
    let result = zyn::Render::render(
        &AttrElement {
            label: zyn::quote::format_ident!("hello"),
        },
        &Default::default(),
    );
    let expected = quote!(
        fn hello_default() {}
    );
    assert_eq!(result.to_string(), expected.to_string());
}
