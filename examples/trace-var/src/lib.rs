mod folder;

use std::collections::HashSet;

use proc_macro::TokenStream;
use zyn::Args;
use zyn::TokenStream as TokenStream2;
use zyn::syn::Expr;
use zyn::syn::Ident;
use zyn::syn::Pat;
use zyn::syn::fold::Fold;

use folder::TraceVarFolder;

#[zyn::element]
fn assign_trace(left: Expr, op: TokenStream2, right: Expr) -> TokenStream2 {
    zyn::zyn!({
        {
            { left }
        }
        {
            { op }
        }
        {
            { right }
        };
        ::std::println!(
            ::std::concat!(::std::stringify!({ { left } }), " = {:?}"),
            { { left } },
        );
    })
}

#[zyn::element]
fn let_trace(pat: Pat, init: Expr, ident: Ident) -> TokenStream2 {
    zyn::zyn!(
        let {{ pat }} = {
            #[allow(unused_mut)]
            let {{ pat }} = {{ init }};
            ::std::println!(
                ::std::concat!(::std::stringify!({{ ident }}), " = {:?}"),
                {{ ident }},
            );
            {{ ident }}
        };
    )
}

#[zyn::element]
fn trace_var_element(#[zyn(input)] item: zyn::syn::ItemFn, vars: HashSet<Ident>) -> TokenStream2 {
    let mut folder = TraceVarFolder { vars: vars.clone() };
    zyn::zyn!({ { folder.fold_item_fn(item) } })
}

#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    let ext_args = zyn::parse_input!(args as Args);
    let vars: HashSet<Ident> = ext_args.iter().filter_map(|a| a.name().cloned()).collect();
    let input: zyn::Input = zyn::Input::Item(zyn::parse_input!(input as zyn::syn::Item));
    zyn::zyn!(
        @trace_var_element(vars = vars)
    )
    .into()
}
