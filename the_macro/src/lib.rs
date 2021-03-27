use std::fmt::Write;

use parse::Parse;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, punctuated::Punctuated};

struct Literals(Punctuated<syn::Lit, syn::Token!(,)>);

impl Parse for Literals {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        input.parse_terminated(syn::Lit::parse).map(Self)
    }
}

/// Concatenates various literals as strings into a string.
///
/// ```rust
/// use the_macro::concatenate;
///
/// const SOMETHING: &str = concatenate!("14", "wow", true);
///
/// assert_eq!(SOMETHING, "14wowtrue");
/// ```
#[proc_macro]
pub fn concatenate(input: TokenStream) -> TokenStream {
    let Literals(literals) = syn::parse_macro_input!(input);
    let mut output = String::new();
    for literal in literals {
        match literal {
            syn::Lit::Str(s) => {
                write!(output, "{}", s.value())
            }
            syn::Lit::ByteStr(s) => {
                write!(output, "{}", String::from_utf8_lossy(&s.value()))
            }
            syn::Lit::Byte(b) => {
                write!(output, "{}", char::from(b.value()))
            }
            syn::Lit::Char(c) => {
                write!(output, "{}", c.value())
            }
            syn::Lit::Int(i) => {
                write!(output, "{}", i.base10_digits())
            }
            syn::Lit::Float(f) => {
                write!(output, "{}", f.base10_digits())
            }
            syn::Lit::Bool(b) => {
                write!(output, "{}", b.value())
            }
            syn::Lit::Verbatim(v) => {
                write!(output, "{}", v)
            }
        }
        .expect("Should not fail");
    }
    quote!(#output).into()
}
