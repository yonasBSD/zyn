use proc_macro2::TokenStream;

pub fn print_pretty(tokens: &TokenStream) {
    let formatted = format_tokens(&tokens.to_string());
    eprintln!("zyn::expand! ─── pretty\n{}", formatted);
}

pub fn format_tokens(input: &str) -> String {
    let mut result = String::new();
    let mut depth: usize = 0;
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                result.push('{');
                result.push('\n');
                depth += 1;
                push_indent(&mut result, depth);
            }
            '}' => {
                depth = depth.saturating_sub(1);
                if result.ends_with("    ") || result.ends_with('\n') {
                    result.truncate(result.trim_end().len());
                    result.push('\n');
                }
                push_indent(&mut result, depth);
                result.push('}');

                if chars
                    .peek()
                    .is_some_and(|c| *c != ',' && *c != ';' && *c != ')')
                {
                    result.push('\n');
                    push_indent(&mut result, depth);
                }
            }
            ';' => {
                result.push(';');
                result.push('\n');
                push_indent(&mut result, depth);
            }
            ' ' if result.ends_with('\n') || result.ends_with("    ") => {}
            _ => {
                result.push(ch);
            }
        }
    }

    result.trim().to_string()
}

fn push_indent(s: &mut String, depth: usize) {
    for _ in 0..depth {
        s.push_str("    ");
    }
}
