macro_rules! syn_bail {
    ($tokens:expr, $msg:expr) => {
        return Err(syn::Error::new_spanned($tokens, $msg))
    };

    ($span:expr => $msg:expr) => {
        return Err(syn::Error::new($span, $msg))
    };
}

pub(crate) use syn_bail;
