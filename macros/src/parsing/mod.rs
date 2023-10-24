mod lexer;
mod parser;
mod span_len;

use crate::parsing::{
    lexer::{Lexer, Token},
    parser::Parser,
};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::LitStr;

fn debug(lexer: Lexer<'_>, raw_text: &str) {
    let parsed_tokens: Vec<_> = lexer.clone().collect();

    println!("{:?}", parsed_tokens);

    for t in parsed_tokens {
        match t {
            Token::Text { pos, len } => {
                let text = &raw_text[pos..(pos + len)];
                println!("Text: {:?} {}", text, text.len())
            },
            Token::Whitespace { pos, len } => {
                println!("Whitespace: {:?}", &raw_text[pos..(pos + len)])
            },
            Token::Newline { pos, len } => {
                println!("Newline: {:?}", &raw_text[pos..(pos + len)])
            },
            Token::Expr { .. } => println!("Expr"),
        }
    }
}

/// `tokens` and `raw_text` have to match
pub fn parse(ts: TokenStream, raw_text: &str) -> TokenStream2 {
    let lexer = Lexer::new(ts, raw_text);

    debug(lexer.clone(), raw_text);

    let mut parser = Parser::new(lexer);

    let mut out = TokenStream2::new();

    loop {
        println!("{:?}", parser.get_cur());
        let Some(str) = parser.parse_lit() else { break };

        println!("{:?}", str);

        if !str.is_empty() {
            out.extend(quote!(template.push_str(#str);))
        }

        let Some((expr, _)) = parser.parse_expr() else { continue };

        out.extend(quote!(template.push_str(#expr.to_string().as_str());))
    }

    out
}
