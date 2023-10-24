use crate::parsing::span_len::SpanLen;
use proc_macro::{token_stream, Delimiter, Group, TokenStream};

#[derive(Debug)]
pub enum Token {
    Text { pos: usize, len: usize },
    Whitespace { pos: usize, len: usize, has_newline: bool },

    Expr(TokenStream),
}

/// Can iterate over the input once.
///
/// # Example
///
/// ```
/// text: "Hello  { name }!"
///
/// ts:    ^^^^^  ^^^^^^^^
///        Ident   Group  ^Punct
///
/// Token: ^^^^^  ^^^^^^^^
///        Text ^^  Expr  ^Text
///             Whitespace
/// ```
#[derive(Clone)]
pub struct Lexer<'a> {
    ts: token_stream::IntoIter,
    /// raw text which produced `ts`.
    text: &'a str,
    /// position in `text`.
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(ts: TokenStream, raw_text: &'a str) -> Self {
        println!("{:#?}", ts);
        println!("{:?}", raw_text);
        Self { ts: ts.into_iter(), text: raw_text, pos: 0 }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.get_byte()?.is_ascii_whitespace() {
            self.next_whitespace()
        } else {
            self.next_tt()
        }
    }

    fn next_tt(&mut self) -> Option<Token> {
        let tt = self.ts.next()?;
        let span = tt.span();

        // NOTE (debug_assert): These should never fail unless `TokenStream` works
        // differently than I expected.

        use proc_macro::TokenTree as TT;
        Some(match tt {
            TT::Ident(i) => {
                let len = span.len();
                let pos = self.pos;
                debug_assert_eq!(i.to_string().as_str(), self.sub_text(pos, len));
                self.pos += len;
                Token::Text { pos, len }
            },
            TT::Punct(p) => {
                let pos = self.pos;
                debug_assert_eq!(p.as_char(), self.get_byte().expect("byte exists") as char);
                self.pos += 1;
                Token::Text { pos, len: 1 }
            },
            TT::Group(g) if is_brace(&g) => {
                self.pos += span.len();
                Token::Expr(g.stream())
            },
            TT::Group(_) => todo!(),
            TT::Literal(_) => todo!(),
        })
    }

    fn next_whitespace(&mut self) -> Option<Token> {
        let count = self.rem_text().find(|c: char| !c.is_ascii_whitespace())?;

        if count == 0 {
            return None;
        }

        let pos = self.pos;

        let has_newline = self.text[pos..(pos + count)].lines().count() > 1;

        self.pos += count;
        Some(Token::Whitespace { pos, len: count, has_newline })
    }

    /// get remaining text
    #[inline]
    fn rem_text(&self) -> &str {
        &self.text.get(self.pos..).unwrap_or("")
    }

    #[inline]
    fn sub_text(&self, pos: usize, len: usize) -> &str {
        &self.text.get(pos..(pos + len)).unwrap_or("")
    }

    #[inline]
    fn get_byte(&self) -> Option<u8> {
        self.text.as_bytes().get(self.pos).copied()
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let ts_hint = self.ts.size_hint();
        // `self.ts` should be a `ExactSizeIterator` -> this can be improved.
        // But I'm not 100% sure.

        // Text before first and after end is ignored.
        (ts_hint.0, ts_hint.1.map(|upper| upper * 2 - 1))
    }
}

fn is_brace(g: &Group) -> bool {
    g.delimiter() == Delimiter::Brace
}
