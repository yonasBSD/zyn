fn main() {
    let _: proc_macro2::TokenStream = zyn::zyn!(
        @throw("this should fail to compile")
    );
}
