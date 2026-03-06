use zyn_core::proc_macro2::TokenStream;
use zyn_core::quote::quote;

pub fn macros() -> TokenStream {
    quote! {
        macro_rules! error {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ))
            };
        }

        macro_rules! warn {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Warning, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Warning, format!($fmt $(, $arg)*)
                ))
            };
        }

        macro_rules! note {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Note, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Note, format!($fmt $(, $arg)*)
                ))
            };
        }

        macro_rules! help {
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Help, format!($fmt $(, $arg)*)
                ))
            };
            ($fmt:literal $(, $arg:expr)* $(,)?) => {
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Help, format!($fmt $(, $arg)*)
                ))
            };
        }

        macro_rules! bail {
            () => {
                if diagnostics.has_errors() {
                    return diagnostics.emit();
                }
            };
            ($fmt:literal $(, $arg:expr)* ; span = $span:expr) => {{
                diagnostics.push(::zyn::Diagnostic::spanned(
                    $span, ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ));
                return diagnostics.emit();
            }};
            ($fmt:literal $(, $arg:expr)* $(,)?) => {{
                diagnostics.push(::zyn::Diagnostic::spanned(
                    ::zyn::Span::call_site(), ::zyn::Level::Error, format!($fmt $(, $arg)*)
                ));
                return diagnostics.emit();
            }};
        }
    }
}
