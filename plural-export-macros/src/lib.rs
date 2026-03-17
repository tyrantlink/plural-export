mod export_conversion;

use proc_macro::TokenStream;

#[proc_macro]
pub fn export_conversion(input: TokenStream) -> TokenStream {
    export_conversion::export_conversion(input)
}
