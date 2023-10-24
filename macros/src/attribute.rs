use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug)]
pub struct TemplateAttribute {
    path: Option<String>,
}

impl TryFrom<TokenStream2> for TemplateAttribute {
    type Error = syn::Error;

    fn try_from(attr: TokenStream2) -> Result<Self, Self::Error> {
        println!("attr: {}", attr);
        todo!("TryFrom<TokenStream2> for TemplateAttribute")
    }
}
