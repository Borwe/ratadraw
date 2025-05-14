use std::str::FromStr;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rund(attr: TokenStream, input: TokenStream) -> TokenStream {
    let result = cargo_rund_derive_impl::do_work(attr.to_string(), input.to_string());
    TokenStream::from_str(&result).expect("parse result from cargo-rund failed")
}
