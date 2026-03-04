use std::collections::HashSet;

use proc_macro::TokenStream;
use zyn::proc_macro2::Ident;
use zyn::proc_macro2::TokenStream as TokenStream2;
use zyn::syn::BinOp;
use zyn::syn::Expr;
use zyn::syn::ExprAssign;
use zyn::syn::ExprBinary;
use zyn::syn::ItemFn;
use zyn::syn::Local;
use zyn::syn::Pat;
use zyn::syn::Stmt;
use zyn::syn::fold;
use zyn::syn::fold::Fold;
use zyn::syn::parse_macro_input;
use zyn::ext::Args;

#[zyn::element]
fn assign_trace(
    left: Expr,
    op: TokenStream2,
    right: Expr,
) -> zyn::syn::Result<TokenStream2> {
    Ok(zyn::zyn!({
        {{ left }} {{ op }} {{ right }};
        ::std::println!(
            ::std::concat!(::std::stringify!({{ left }}), " = {:?}"),
            {{ left }},
        );
    }))
}

#[zyn::element]
fn let_trace(
    pat: Pat,
    init: Expr,
    ident: Ident,
) -> zyn::syn::Result<TokenStream2> {
    Ok(zyn::zyn!(
        let {{ pat }} = {
            #[allow(unused_mut)]
            let {{ pat }} = {{ init }};
            ::std::println!(
                ::std::concat!(::std::stringify!({{ ident }}), " = {:?}"),
                {{ ident }},
            );
            {{ ident }}
        };
    ))
}

fn traced_assign(left: Expr, op: TokenStream2, right: Expr) -> zyn::syn::Result<Expr> {
    zyn::syn::parse2(zyn::zyn!(
        @assign_trace(left = left, op = op, right = right)
    ))
}

fn traced_let(pat: Pat, init: Expr, ident: Ident) -> zyn::syn::Result<Stmt> {
    zyn::syn::parse2(zyn::zyn!(
        @let_trace(pat = pat, init = init, ident = ident)
    ))
}

struct TraceVar {
    vars: HashSet<Ident>,
}

impl TraceVar {
    fn is_traced_expr(&self, e: &Expr) -> bool {
        match e {
            Expr::Path(e) => {
                e.path.leading_colon.is_none()
                    && e.path.segments.len() == 1
                    && {
                        let seg = e.path.segments.first().unwrap();
                        self.vars.contains(&seg.ident) && seg.arguments.is_empty()
                    }
            }
            _ => false,
        }
    }

    fn is_traced_pat(&self, p: &Pat) -> bool {
        match p {
            Pat::Ident(p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }
}

impl Fold for TraceVar {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Assign(ExprAssign { attrs, left, eq_token, right }) => {
                let right = fold::fold_expr(self, *right);
                let left = *left;
                let op = zyn::zyn!({{ eq_token }});
                if self.is_traced_expr(&left) {
                    traced_assign(left, op, right).unwrap()
                } else {
                    Expr::Assign(ExprAssign {
                        attrs,
                        left: Box::new(left),
                        eq_token,
                        right: Box::new(right),
                    })
                }
            }
            Expr::Binary(ref bin) if is_assign_op(bin.op) => {
                if let Expr::Binary(ExprBinary { attrs, left, op, right }) = e.clone() {
                    let right = fold::fold_expr(self, *right);
                    let left = *left;
                    let op_ts = zyn::zyn!({{ op }});
                    if self.is_traced_expr(&left) {
                        traced_assign(left, op_ts, right).unwrap()
                    } else {
                        Expr::Binary(ExprBinary {
                            attrs,
                            left: Box::new(left),
                            op,
                            right: Box::new(right),
                        })
                    }
                } else {
                    unreachable!()
                }
            }
            _ => fold::fold_expr(self, e),
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(ref local) if local.init.is_some() && self.is_traced_pat(&local.pat) => {
                let Stmt::Local(local) = s else { unreachable!() };
                let Local { pat, init, .. } = local;
                let init_expr = fold::fold_expr(self, *init.unwrap().expr);
                let ident = match &pat {
                    Pat::Ident(p) => p.ident.clone(),
                    _ => unreachable!(),
                };
                traced_let(pat, init_expr, ident).unwrap()
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

fn is_assign_op(op: BinOp) -> bool {
    matches!(
        op,
        BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
    )
}

#[proc_macro_attribute]
pub fn trace_var(args: TokenStream, input: TokenStream) -> TokenStream {
    let ext_args = parse_macro_input!(args as Args);
    let vars: HashSet<Ident> = ext_args
        .iter()
        .filter_map(|a| a.name().cloned())
        .collect();
    let input = parse_macro_input!(input as ItemFn);
    let mut folder = TraceVar { vars };
    let output = folder.fold_item_fn(input);
    zyn::zyn!({{ output }}).into()
}
