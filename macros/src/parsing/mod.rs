use proc_macro::{Delimiter, Group, Span, TokenStream};

#[derive(Debug)]
pub enum Token {
    Text { pos: usize, len: usize },
    Whitespace { pos: usize, len: usize, has_newline: bool },

    Expr(TokenStream),
}

fn is_brace(g: &Group) -> bool {
    g.delimiter() == Delimiter::Brace
}

/// why is this not part of `proc_macro`?
fn get_span_len(span: Span) -> usize {
    span.source_text()
        .map(|s| s.len())
        .expect("span text should exist")
}

/// `tokens` and `raw_text` have to match
pub fn parse(ts: TokenStream, raw_text: &str) {
    println!("{:#?}", ts);
    println!("{:#?}", raw_text);

    let mut pos = 0;

    println!("from ts: {}", ts.to_string());
    println!("raw    : {}", raw_text);

    let tokens = ts.into_iter();
    let len = tokens.size_hint().0 * 2;

    let mut parsed_tokens = Vec::with_capacity(len);

    for tt in tokens {
        let span = tt.span();

        use proc_macro::TokenTree as TT;
        match tt {
            TT::Ident(i) => {
                let len = get_span_len(span);
                debug_assert_eq!(i.to_string().as_str(), &raw_text[pos..(pos + len)]);
                parsed_tokens.push(Token::Text { pos, len });
                pos += len;
            },
            TT::Punct(p) => {
                debug_assert_eq!(p.as_char(), raw_text.as_bytes()[pos] as char);
                parsed_tokens.push(Token::Text { pos, len: 1 });
                pos += 1;
            },
            TT::Group(g) if is_brace(&g) => {
                parsed_tokens.push(Token::Expr(g.stream()));
                pos += get_span_len(span);
            },
            TT::Group(_) => todo!(),
            TT::Literal(_) => todo!(),
        }

        let whitespace = raw_text[pos..]
            .find(|c: char| !c.is_whitespace())
            .unwrap_or(0);

        if whitespace == 0 {
            continue
        }

        let has_newline = raw_text[pos..(pos + whitespace)].lines().count() > 1;

        parsed_tokens.push(Token::Whitespace { pos, len: whitespace, has_newline });
        pos += whitespace;
    }


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
}
