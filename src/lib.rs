// We need to support Rust 1.34 to stable
#![allow(deprecated)]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate syn;

#[macro_use]
extern crate quote;

mod ast;
mod attr;
mod bound;
mod clone;
mod cmp;
mod debug;
mod default;
mod hash;
mod matcher;
mod paths;
mod utils;

use proc_macro::TokenStream;

fn derive_impls(input: &mut ast::Input) -> Result<proc_macro2::TokenStream, String> {
    let mut tokens = proc_macro2::TokenStream::new();

    if input.attrs.clone.is_some() {
        tokens.extend(clone::derive_clone(input));
    }
    if input.attrs.copy.is_some() {
        tokens.extend(clone::derive_copy(input));
    }
    if input.attrs.debug.is_some() {
        tokens.extend(debug::derive(input));
    }
    if let Some(ref default) = input.attrs.default {
        tokens.extend(default::derive(input, default));
    }
    if input.attrs.eq.is_some() {
        tokens.extend(cmp::derive_eq(input));
    }
    if input.attrs.hash.is_some() {
        tokens.extend(hash::derive(input));
    }
    if input.attrs.partial_eq.is_some() {
        tokens.extend(cmp::derive_partial_eq(input)?);
    }
    if input.attrs.partial_ord.is_some() {
        tokens.extend(cmp::derive_partial_ord(input)?);
    }
    if input.attrs.ord.is_some() {
        tokens.extend(cmp::derive_ord(input)?);
    }

    tokens.extend(std::mem::replace(&mut input.attrs.errors, Default::default()));

    Ok(tokens)
}

fn detail(input: TokenStream) -> Result<TokenStream, String> {
    let parsed = syn::parse::<syn::DeriveInput>(input).map_err(|e| e.to_string())?;
    let output = derive_impls(&mut ast::Input::from_ast(&parsed)?)?;
    Ok(output.into())
}

#[cfg_attr(not(test), proc_macro_derive(Derivative, attributes(derivative)))]
pub fn derivative(input: TokenStream) -> TokenStream {
    match detail(input) {
        Ok(output) => output,
        Err(e) => panic!(e),
    }
}
