mod folder;

use std::collections::HashSet;

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

#[zyn::attribute]
fn trace_var(#[zyn(input)] item: zyn::syn::ItemFn, args: zyn::Args) -> TokenStream2 {
    let vars: HashSet<Ident> = args.iter().filter_map(|a| a.name().cloned()).collect();
    let mut folder = TraceVarFolder { input, vars };
    zyn::zyn!({ { folder.fold_item_fn(item) } })
}
