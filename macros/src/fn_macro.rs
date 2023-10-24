use crate::{parsing, syn_bail::syn_bail};
use proc_macro::TokenStream;
use proc_macro2::{Span as Span2, TokenStream as TokenStream2};
use quote::quote;

pub fn expand_templ(ts: TokenStream) -> syn::Result<TokenStream2> {
    // `Span2::mixed_site()` should give the same text.
    let macro_span = Span2::call_site();
    let Some(text) = macro_span.source_text() else {
        syn_bail!(macro_span => "Span doesn't correspond to real source code")
    };

    let mut chars = text.char_indices().skip_while(|c| c.1 != '!');
    if !matches!(chars.next(), Some((_, '!'))) {
        syn_bail!(macro_span => "Cannot find the '!' of the macro")
    }

    // macros can be called with different delimiters
    let (body_start, closing_delim) = match chars.skip_while(|c| c.1.is_ascii_whitespace()).next() {
        Some((pos, '{')) => (pos + 1, '}'), // templ! { ... }
        Some((pos, '(')) => (pos + 1, ')'), // templ!(...);
        Some((pos, '[')) => (pos + 1, ']'), // templ![...];
        Some((_, c)) => syn_bail!(macro_span => format!("{c:?} is not a macro delimiter")),
        None => syn_bail!(macro_span => "Cannot find the opening delimiter of the macro"),
    };

    if text.strip_suffix(closing_delim).is_none() {
        syn_bail!(macro_span => "Cannot find the closing delimiter of the macro")
    }

    let end = text.len() - 1;
    let macro_body = text[body_start..end].trim();

    let _out = parsing::parse(ts, macro_body);

    Ok(quote! {
        {
            let mut template = String::new();
            //template.push_str(#lit);
            template
        }
    })
}
