use super::{
    lexer::{Lexer, Token},
    parse,
};
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::ToTokens;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    cur_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Parser<'a> {
        let cur_token = lexer.next();
        Self { lexer, cur_token }
    }

    pub fn get_cur(&self) -> Option<&Token> {
        self.cur_token.as_ref()
    }

    /*
    fn next_token(&mut self) -> Option<Token> {
        let token = self.cur_token.take();
        self.cur_token = self.lexer.next();
        token
    }

    fn peek(&self) -> Option<&Token> {
        self.cur_token.as_ref()
    }
    */

    /// Parse the next [`Token`]s as a string literal. If the [`Parser`] is
    /// empty, this returns [`None`].
    /// If the next token isn't part of a literal, this also returns [`None`]
    /// and `self` isn't modified.
    pub fn parse_lit(&mut self) -> Option<String> {
        let token = match self.cur_token.take()? {
            Token::Text { pos, len } | Token::Whitespace { pos, len } => {
                &self.lexer.text[pos..pos + len]
            },
            Token::Newline { pos, len } => " ", // replace whitespace with newline
            t => {
                self.cur_token = Some(t);
                return Some("".to_string());
            },
        };
        let mut text = token.to_string();

        loop {
            match self.lexer.next() {
                Some(Token::Text { pos, len } | Token::Whitespace { pos, len }) => {
                    text.push_str(&self.lexer.text[pos..pos + len])
                },
                Some(Token::Newline { .. }) => text.push_str(" "),
                t => {
                    self.cur_token = t;
                    break;
                },
            }
        }

        Some(text)
    }

    fn _parse_lit_with_nl(&mut self) -> Option<&str> {
        let mut cur = self.cur_token.take()?;
        if !cur.is_lit() {
            self.cur_token = Some(cur);
            return Some("");
        }
        let start = cur.get_pos();

        loop {
            match self.lexer.next() {
                Some(t) if t.is_lit() => cur = t,
                t => {
                    self.cur_token = t;
                    break;
                },
            }
        }

        let end = cur.get_end();

        self.lexer.text.get(start..end)
    }

    pub fn parse_expr(&mut self) -> Option<(TokenStream2, Span2)> {
        match self.cur_token.take()? {
            Token::Expr { ts, span, .. } => {
                self.cur_token = self.lexer.next();
                Some((ts.into(), span.into()))
            },
            t => {
                self.cur_token = Some(t);
                None
            },
        }
    }
}
