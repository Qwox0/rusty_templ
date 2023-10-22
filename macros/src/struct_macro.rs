use crate::{attribute::TemplateAttribute, syn_bail::syn_bail};
use proc_macro::{Delimiter, Group, Span, TokenStream, TokenTree};
use proc_macro2::{Literal, Span as Span2, TokenStream as TokenStream2, TokenTree as TokenTree2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse_macro_input, punctuated::Punctuated, Attribute, DataStruct, DeriveInput, ExprLit,
    ItemStruct, Lit, LitStr, Meta, Token,
};

pub fn expand_template(attr: TokenStream, item: ItemStruct) -> TokenStream {
    let attr: TokenStream2 = attr.into();
    let mut out = item.to_token_stream();

    let trait_impl = TemplateAttribute::try_from(attr)
        .and_then(|attr| impl_template(attr, item))
        .unwrap_or_else(syn::Error::into_compile_error);

    out.extend(trait_impl);
    out.into()
}

pub fn expand_derive_template(input: DeriveInput) -> syn::Result<TokenStream2> {
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
                syn::Expr::Lit(ExprLit { lit: Lit::Str(lit), .. }) => lit.parse::<TokenTree2>(),
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

pub fn impl_template(attr: TemplateAttribute, item: ItemStruct) -> syn::Result<TokenStream2> {
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
