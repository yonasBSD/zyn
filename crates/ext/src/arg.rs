use proc_macro2::Ident;

use quote::ToTokens;
use quote::TokenStreamExt;

use syn::Expr;
use syn::Lit;
use syn::Token;
use syn::parse::Parse;
use syn::parse::ParseStream;

use crate::Args;

pub enum Arg {
    Flag(Ident),
    Expr(Ident, Expr),
    List(Ident, Args),
    Lit(Lit),
}

impl Arg {
    pub fn name(&self) -> Option<&Ident> {
        match self {
            Self::Flag(name) => Some(name),
            Self::Expr(name, _) => Some(name),
            Self::List(name, _) => Some(name),
            Self::Lit(_) => None,
        }
    }

    pub fn is_flag(&self) -> bool {
        matches!(self, Self::Flag(_))
    }

    pub fn is_expr(&self) -> bool {
        matches!(self, Self::Expr(_, _))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_, _))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn as_expr(&self) -> &Expr {
        match self {
            Self::Expr(_, expr) => expr,
            _ => panic!("called `Arg::as_expr()` on a non-Expr variant"),
        }
    }

    pub fn as_args(&self) -> &Args {
        match self {
            Self::List(_, args) => args,
            _ => panic!("called `Arg::as_args()` on a non-List variant"),
        }
    }

    pub fn as_lit(&self) -> &Lit {
        match self {
            Self::Lit(lit) => lit,
            _ => panic!("called `Arg::as_lit()` on a non-Lit variant"),
        }
    }
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Lit) || input.peek(syn::LitStr) {
            return Ok(Self::Lit(input.parse()?));
        }

        let name: Ident = input.parse()?;

        if input.peek(Token![=]) {
            input.parse::<Token![=]>()?;
            let expr: Expr = input.parse()?;
            Ok(Self::Expr(name, expr))
        } else if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let args: Args = content.parse()?;
            Ok(Self::List(name, args))
        } else {
            Ok(Self::Flag(name))
        }
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Flag(name) => name.to_tokens(tokens),
            Self::Expr(name, expr) => {
                name.to_tokens(tokens);
                tokens.append(proc_macro2::Punct::new('=', proc_macro2::Spacing::Alone));
                expr.to_tokens(tokens);
            }
            Self::List(name, args) => {
                name.to_tokens(tokens);
                let inner = args.to_token_stream();
                tokens.append(proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    inner,
                ));
            }
            Self::Lit(lit) => lit.to_tokens(tokens),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse {
        use super::*;

        #[test]
        fn flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            assert!(arg.is_flag());
            assert_eq!(arg.name().unwrap(), "skip");
        }

        #[test]
        fn expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            assert!(arg.is_expr());
            assert_eq!(arg.name().unwrap(), "rename");
        }

        #[test]
        fn list() {
            let arg: Arg = syn::parse_str("serde(rename_all = \"camelCase\")").unwrap();
            assert!(arg.is_list());
            assert_eq!(arg.name().unwrap(), "serde");
            assert!(arg.as_args().has("rename_all"));
        }

        #[test]
        fn lit_str() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            assert!(arg.is_lit());
            assert!(arg.name().is_none());
        }

        #[test]
        fn lit_int() {
            let arg: Arg = syn::parse_str("42").unwrap();
            assert!(arg.is_lit());
        }
    }

    mod to_tokens {
        use super::*;

        #[test]
        fn flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "skip");
        }

        #[test]
        fn expr() {
            let arg: Arg = syn::parse_str("rename = \"foo\"").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "rename = \"foo\"");
        }

        #[test]
        fn list() {
            let arg: Arg = syn::parse_str("serde(skip)").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "serde (skip)");
        }

        #[test]
        fn lit() {
            let arg: Arg = syn::parse_str("\"hello\"").unwrap();
            let output = arg.to_token_stream().to_string();
            assert_eq!(output, "\"hello\"");
        }
    }

    mod accessors {
        use super::*;

        #[test]
        #[should_panic(expected = "non-Expr")]
        fn as_expr_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_expr();
        }

        #[test]
        #[should_panic(expected = "non-List")]
        fn as_args_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_args();
        }

        #[test]
        #[should_panic(expected = "non-Lit")]
        fn as_lit_panics_on_flag() {
            let arg: Arg = syn::parse_str("skip").unwrap();
            arg.as_lit();
        }
    }
}
