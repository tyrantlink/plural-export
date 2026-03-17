use proc_macro2::TokenStream;
use quote::{ToTokens, quote};


pub fn option_to_tokens<T: ToTokens>(option: Option<&T>) -> TokenStream {
    option.map_or_else(|| quote! { None }, |option| quote! { Some(#option) })
}
