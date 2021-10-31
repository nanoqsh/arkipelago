mod layout;
mod shader_set;
mod uniforms;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Layout)]
pub fn layout(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    layout::impl_macro(&input).into()
}

#[proc_macro_attribute]
pub fn uniforms(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    uniforms::impl_macro(&input).into()
}

#[proc_macro_attribute]
pub fn shader_set(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    shader_set::impl_macro(&input).into()
}
