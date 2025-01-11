#![feature(proc_macro_diagnostic)]
use quote::{
    quote,
    quote_spanned,
};
use syn::{
    parse::Parse,
    parse_macro_input,
    spanned::Spanned,
    Expr,
    Ident,
    Token,
    Type,
    Visibility,
};

use proc_macro2::TokenStream;
use proc_macro2::Span;

struct LazyStatic {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    init: Expr,
}

impl Parse for LazyStatic {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let visibility: Visibility = input.parse()?;
        input.parse::<Token![static]>()?;
        input.parse::<Token![ref]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: Expr = input.parse()?;

        Ok(LazyStatic {
            visibility,
            name,
            ty,
            init,
        })
    }
}

#[proc_macro]
pub fn lazy_static(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let LazyStatic {
        visibility,
        name,
        ty,
        init,
    } = parse_macro_input!(input as LazyStatic);

    if name == "FOO" {
        name.span()
            .unwrap()
            .warning("come on, pick a more creative name")
            .emit();
    }

    if let Expr::Tuple(ref init) = init {
        if init
            .elems
            .is_empty()
        {
            init.span().unwrap().error("I can't think of a legitimate use fof lazily initializing the value`()`").emit();
            return proc_macro::TokenStream::new();
        }
    }

    let assert_sync = quote_spanned! {
        ty.span()=> struct _AssertSync where #ty: std::marker::Sync;
    };

    let assert_sized = quote_spanned! {
        ty.span()=> struct _AssertSized where #ty: std::marker::Sized;
    };

    let init_ptr = quote_spanned! {
        init.span()=> Box::into_raw(Box::new(#init))
    };

    let expanded = quote! {
        #visibility struct #name;

        impl std::ops::Deref for #name {
            type Target = #ty;

            fn deref(&self) -> &#ty {
                #assert_sync
                #assert_sized

                static ONCE: std::sync::Once = std::sync::Once::new();
                static mut VALUE: *mut #ty = 0 as *mut #ty;

                unsafe {
                    ONCE.call_once(||VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
