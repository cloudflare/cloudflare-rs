extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ApiResult)]
pub fn api_result_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl crate::framework::response::ApiResult for #name {}
    };

    gen.into()
}

#[proc_macro_derive(VecApiResult)]
pub fn vec_api_result_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let gen = quote! {
        impl crate::framework::response::ApiResult for Vec<#name> {}
    };

    gen.into()
}
