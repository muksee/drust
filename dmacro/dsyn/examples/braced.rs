use std::str::FromStr;

use proc_macro2::{
    Ident,
    TokenStream,
};
use syn::{
    braced,
    parse::Parse,
    parse2,
    punctuated::Punctuated,
    token,
    Token,
    Type,
};

#[allow(unused)]
struct Struct {
    struct_token: token::Struct,
    ident: Ident,
    brace_token: token::Brace,
    fields: Punctuated<Field, Token![,]>,
}

#[allow(unused)]
struct Field {
    name: Ident,
    colon_token: Token![:],
    ty: Type,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Field {
            name: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
        })
    }
}

impl Parse for Struct {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Struct {
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!( content in input), // bracketed, parenthesized
            fields: content.parse_terminated(Field::parse, Token![,])?,
        })
    }
}

fn main() {
    let input = r#"
    struct S { a: A, b: B, }
    "#;
    let ts = TokenStream::from_str(input).unwrap();
    let s: Struct = parse2(ts).unwrap();
}
