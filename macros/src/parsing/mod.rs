mod lexer;
mod parser;
mod span_len;

use crate::parsing::{lexer::Lexer, parser::Parser};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

/// `tokens` and `raw_text` have to match
pub fn parse(ts: TokenStream, raw_text: &str) -> TokenStream2 {
    let lexer = Lexer::new(ts, raw_text);

    let mut parser = Parser::new(lexer);

    let mut out = TokenStream2::new();

    loop {
        let Some(str) = parser.parse_lit() else { break };

        if !str.is_empty() {
            out.extend(quote!(template.push_str(#str);))
        }

        let Some((expr, _)) = parser.parse_expr() else { continue };

        out.extend(quote!(template.push_str(#expr.to_string().as_str());))
    }

    out
}
