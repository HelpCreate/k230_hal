extern crate proc_macro;
use std::{env, fs};


use proc_macro::TokenStream;

const CONFIG_FILE: &str = include_str!(env!("K230_CROSS_CORE_CONFIG_FILE","you need to specify the path to the config file in the environment variable K230_CROSS_CORE_CONFIG_FILE"));
#[proc_macro]
pub fn test(_item: TokenStream) -> TokenStream {
    panic!("{}", CONFIG_FILE);
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
