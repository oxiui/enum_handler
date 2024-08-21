#![doc = include_str!("../README.md")]

use enum_handler_core::enum_handler_core;

#[proc_macro_derive(EnumHandler, attributes(enum_handler))]
pub fn enum_handler(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input2 = proc_macro2::TokenStream::from(input.clone());
    let result = enum_handler_core(input2);
    match result {
        Ok(result) => proc_macro::TokenStream::from(result),
        Err(e) => panic!("{}", e),
    }
}
