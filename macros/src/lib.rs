#![feature(proc_macro_span)]
#![feature(let_chains)]
#![feature(extend_one)]

mod attribute;
mod output_steam;
mod syn_bail;

use crate::output_steam::OutputStream;
use attribute::TemplateAttribute;
use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};
use proc_macro2::{Literal, Span as Span2, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DataStruct, DeriveInput, ExprLit,
    ItemStruct, Lit, LitStr, Meta, Token,
};
use syn_bail::syn_bail;

fn parse_str_lit(str: LitStr) -> syn::Result<TokenStream2> {
    str.parse()
}

fn is_brace(g: &Group) -> bool {
    g.delimiter() == Delimiter::Brace
}

#[proc_macro]
pub fn templ(item: TokenStream) -> TokenStream {
    let mut output = OutputStream::new();

    println!("{:#?}", item);
    println!("{}", item);

    let mut lit_text = String::new();
    let mut lit_span: Option<Span> = None;

    for tt in item {
        let span = tt.span();
        if let Some(lit_span) = lit_span {
            let whitespace = span.byte_range().start - lit_span.byte_range().end;
            lit_text.push_str(" ".repeat(whitespace).as_str())
        }
        lit_text.push_str(tt.to_string().as_str());
        lit_span = Some(
            lit_span
                .and_then(|lit_span| lit_span.join(span))
                .unwrap_or(span),
        )
    }

    let lit_span = lit_span.map(Into::into).unwrap();
    let lit = LitStr::new(lit_text.as_str(), lit_span);

    quote! {
        {
            let mut template = String::new();
            template.push_str(#lit);
            template
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn template(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    let attr: TokenStream2 = attr.into();
    //let attr = parse_macro_input!(attr with Punctuated<Meta, Token![,]>::parse_terminated);
    //println!("item struct: {:#?}", item);

    let mut out = item.to_token_stream();

    let trait_impl = TemplateAttribute::try_from(attr)
        .and_then(|attr| impl_template(attr, item))
        .unwrap_or_else(syn::Error::into_compile_error);

    out.extend(trait_impl);
    out.into()
}

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    _derive_template(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn _derive_template(input: DeriveInput) -> syn::Result<TokenStream2> {
    let item = match input.data {
        syn::Data::Struct(s) => s,
        syn::Data::Enum(_) => syn_bail!(input, "rusty_template does not support enums"),
        syn::Data::Union(_) => syn_bail!(input, "rusty_template does not support unions"),
    };

    let is_matching_attr = |a: &Attribute| a.path().is_ident("template");
    let mut attr_iter = input.attrs.into_iter().filter(is_matching_attr);

    let Some(attr) = attr_iter.next() else {
        syn_bail!(Span2::call_site() => "A `template` attribute is required")
    };

    if attr_iter.next().is_some() {
        syn_bail!(Span2::call_site() => "Found multiple `template attributes")
    }

    let attr = match attr.meta {
        Meta::Path(_) => todo!(),  // #[template]
        Meta::List(m) => m.tokens, // #[template(...)]
        Meta::NameValue(m) => {
            let l = match m.value {
                syn::Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => parse_str_lit(lit),
                _ => todo!(),
            };
            println!("l: {:?}", l);
            todo!()
        }, // #[template = ...]
    };

    let attr = TemplateAttribute::try_from(attr)?;

    let DeriveInput { vis, ident, generics, .. } = input;
    let DataStruct { struct_token, fields, semi_token } = item;
    let attrs = vec![];
    let item = ItemStruct { attrs, vis, struct_token, ident, generics, fields, semi_token };

    impl_template(attr, item)
}

fn impl_template(attr: TemplateAttribute, item: ItemStruct) -> syn::Result<TokenStream2> {
    println!("item: {:#?}", item);

    let struct_name = &item.ident;

    let fields = item
        .fields
        .iter()
        .enumerate()
        .map(|(idx, field)| match &field.ident {
            Some(f) => f.into_token_stream(),
            None => {
                let var = format_ident!("_{idx}");
                quote!(#idx : #var)
            },
        });

    println!("item: `{}`", item.to_token_stream().to_string());
    println!("attrs: `{:?}`", attr);

    Ok(quote! {
        impl rusty_template::Template for #struct_name {
            fn render(&self) -> String {
                #[allow(unused_variables)]
                let #struct_name { #(#fields),* } = self;
                rusty_template::templ!()
            }
        }
    })
}

#[proc_macro_attribute]
pub fn mytest(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("!!!!!!!!!!!!!!!!!!!!!");
    /*
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());

    let item = syn::parse_macro_input!(item as ItemStruct);

    item.into_token_stream().into()
    */
    item
}
