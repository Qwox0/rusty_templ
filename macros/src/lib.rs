#![feature(proc_macro_span)]
#![feature(let_chains)]
#![feature(extend_one)]

mod attribute;
mod fn_macro;
mod parsing;
mod struct_macro;
mod syn_bail;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

#[proc_macro]
pub fn templ(item: TokenStream) -> TokenStream {
    fn_macro::expand_templ(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_attribute]
pub fn template(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemStruct);
    struct_macro::expand_template(attr, item)
}

#[proc_macro_derive(Template, attributes(template))]
pub fn derive_template(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    struct_macro::expand_derive_template(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
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
