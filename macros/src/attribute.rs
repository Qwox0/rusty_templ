use crate::syn_bail::syn_bail;
use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};
use proc_macro2::{Literal, Span as Span2, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DataStruct, DeriveInput, ExprLit,
    ItemStruct, Lit, LitStr, Meta, Token,
};

#[derive(Debug)]
pub struct TemplateAttribute {
    path: Option<String>,
}

impl TryFrom<TokenStream2> for TemplateAttribute {
    type Error = syn::Error;

    fn try_from(attr: TokenStream2) -> Result<Self, Self::Error> {
        println!("attr: {}", attr);
        todo!()
    }
}
