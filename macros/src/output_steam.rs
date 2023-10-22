use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::ToTokens;
use syn::LitStr;

#[derive(Debug)]
pub struct OutputStream {
    stream: TokenStream2,
}

impl OutputStream {
    pub fn new() -> Self {
        OutputStream { stream: TokenStream2::new() }
    }

    pub fn add_str_lit(&mut self, str_lit: LitStr) {
        self.stream.extend(str_lit.into_token_stream())
    }

    pub fn push_token(&mut self, tt: TokenTree2) {
        self.stream.extend_one(tt)
    }
}

impl Into<TokenStream> for OutputStream {
    fn into(self) -> TokenStream {
        self.stream.into()
    }
}

impl std::fmt::Display for OutputStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.stream.fmt(f)
    }
}
