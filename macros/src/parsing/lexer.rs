use crate::parsing::span_len::SpanLen;
use proc_macro::{token_stream, Delimiter, Group, TokenStream, Span};
use std::iter::FusedIterator;

#[derive(Debug)]
pub enum Token {
    Text { pos: usize, len: usize },
    Whitespace { pos: usize, len: usize },
    Newline { pos: usize, len: usize },

    Expr { pos: usize, ts: TokenStream, span: Span },
}

impl Token {
    fn whitespace(pos: usize, len: usize, has_newline: bool) -> Token {
        match has_newline {
            true => Token::Newline { pos, len },
            false => Token::Whitespace { pos, len },
        }
    }

    pub fn get_pos(&self) -> usize {
        match self {
            Token::Text { pos, .. } => *pos,
            Token::Whitespace { pos, .. } => *pos,
            Token::Newline { pos, .. } => *pos,
            Token::Expr { pos, .. } => *pos,
        }
    }

    pub fn get_end(&self) -> usize {
        match self {
            Token::Text { pos, len } => pos + len,
            Token::Whitespace { pos, len } => pos + len,
            Token::Newline { pos, len } => pos + len,
            Token::Expr { .. } => panic!("no get_end for Token::Expr")
        }
    }

    /// whether `self` is part of a string literal.
    pub fn is_lit(&self) -> bool {
        match self {
            Token::Text { .. } | Token::Whitespace { .. } | Token::Newline { .. } => true,
            Token::Expr { .. } => false,
        }
    }
}

/// Can iterate over the input once.
///
/// # Example
///
/// ```
/// text:        "Hello  { name }!"
///
/// ts:           ^^^^^  ^^^^^^^^
///               Ident   Group  ^Punct
///
/// Lexer output: ^^^^^  ^^^^^^^^
///               Text ^^  Expr  ^Text
///                    Whitespace
/// ```
///
/// # [`Iterator`] Note
///
/// [`proc_macro::token_stream::IntoIter`] only contains a single
/// [`std::vec::IntoIter`] which implements all four Iterator traits
///
/// * [`Iterator`]
/// * [`DoubleEndedIterator`]
/// * [`ExactSizeIterator`]
/// * [`FusedIterator`]
///
/// for any generic type `T`. See <https://doc.rust-lang.org/src/proc_macro/lib.rs.html#364-372>.
///
/// This means that [`proc_macro::token_stream::IntoIter`] could/should also
/// implement all the taits above. This assumption is used for the
/// implementation of [`Iterator::size_hint`] and [`FusedIterator`].
#[derive(Clone)]
pub struct Lexer<'a> {
    ts: token_stream::IntoIter,
    /// raw text which produced `ts`.
    pub(crate) text: &'a str,
    /// position in `text`.
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(ts: TokenStream, raw_text: &'a str) -> Self {
        // println!("{:#?}", ts);
        // println!("{:?}", raw_text);
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

        let pos = self.pos;

        // NOTE (debug_assert): These should never fail unless `TokenStream` works
        // differently than I expected.

        use proc_macro::TokenTree as TT;
        Some(match tt {
            TT::Ident(i) => {
                let len = span.len();
                self.pos += len;
                debug_assert_eq!(Some(i.to_string().as_str()), self.text.get(pos..self.pos));
                Token::Text { pos, len }
            },
            TT::Punct(p) => {
                debug_assert_eq!(p.as_char(), self.get_byte().expect("byte exists") as char);
                self.pos += 1;
                Token::Text { pos, len: 1 }
            },
            TT::Group(g) if is_brace(&g) => {
                self.pos += span.len();
                Token::Expr { pos, ts: g.stream(), span }
            },
            TT::Group(_) => todo!("Lexer: TT::Group"),
            TT::Literal(_) => todo!("Lexer: TT::Literal"),
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
        Some(Token::whitespace(pos, count, has_newline))
    }

    /// get remaining text
    #[inline]
    fn rem_text(&self) -> &str {
        &self.text.get(self.pos..).unwrap_or("")
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
        // see "Iterator Note" on [`Lexer`].
        let ts_size = self.ts.size_hint().0;
        // There *might* be whitespace [`Token`]s between the elements of `ts`.
        (ts_size, ts_size.checked_mul(2).map(|x| x.saturating_sub(1)))
    }
}

impl<'a> FusedIterator for Lexer<'a> {}

fn is_brace(g: &Group) -> bool {
    g.delimiter() == Delimiter::Brace
}
