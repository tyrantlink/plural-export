use std::sync::LazyLock;

use proc_macro_crate::{FoundCrate, crate_name as _crate_name};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::Ident;

static ROOT: LazyLock<FoundCrate> =
    LazyLock::new(|| _crate_name("antcore").expect("antcore crate not found"));

pub struct Antcore;

impl ToTokens for Antcore {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match &*ROOT {
            FoundCrate::Itself => quote! { ::antcore },
            FoundCrate::Name(name) => {
                let ident = Ident::new(name, Span::call_site());
                quote! { ::#ident }
            }
        });
    }
}
