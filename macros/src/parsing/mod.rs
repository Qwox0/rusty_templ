mod lexer;
mod parser;
mod span_len;

use crate::parsing::lexer::{Lexer, Token};
use proc_macro::TokenStream;

/// `tokens` and `raw_text` have to match
pub fn parse(ts: TokenStream, raw_text: &str) {
    let lexer = Lexer::new(ts, raw_text);

    let parsed_tokens: Vec<_> = lexer.collect();

    println!("{:?}", parsed_tokens);

    for t in parsed_tokens {
        match t {
            Token::Text { pos, len } => {
                println!("Text: {:?}", &raw_text[pos..(pos + len)])
            },
            Token::Whitespace { pos, len, has_newline } => {
                println!("Whitespace: {:?} {has_newline}", &raw_text[pos..(pos + len)])
            },
            Token::Expr(_) => (),
        }
    }

    //todo!()
}
