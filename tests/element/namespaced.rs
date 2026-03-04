use zyn::quote::quote;

pub mod components {
    #[zyn::element]
    pub fn field_decl(
        name: zyn::proc_macro2::Ident,
        ty: zyn::proc_macro2::Ident,
    ) -> zyn::proc_macro2::TokenStream {
        zyn::zyn!({{ name }}: {{ ty }},)
    }
}

#[test]
fn namespaced_element() {
    let result = zyn::zyn!(
        @components::field_decl(
            name = zyn::quote::format_ident!("age"),
            ty = zyn::quote::format_ident!("u32"),
        )
    );
    let expected = quote!(age: u32,);
    assert_eq!(result.to_string(), expected.to_string());
}
