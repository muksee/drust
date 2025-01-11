use proc_macro::TokenStream;
use syn::{
    meta::ParseNestedMeta,
    parse_macro_input,
    LitStr,
    Path,
};

#[derive(Default)]
struct TeaAttributes {
    kind: Option<LitStr>,
    hot: bool,
    with: Vec<Path>,
}

impl TeaAttributes {
    fn parse(
        &mut self,
        meta: ParseNestedMeta,
    ) -> Result<(), syn::parse::Error> {
        if meta
            .path
            .is_ident("kind")
        {
            self.kind = Some(
                meta.value()?
                    .parse()?,
            );
            Ok(())
        } else if meta
            .path
            .is_ident("hot")
        {
            self.hot = true;
            Ok(())
        } else if meta
            .path
            .is_ident("with")
        {
            meta.parse_nested_meta(|meta| {
                self.with
                    .push(meta.path);
                Ok(())
            })
        } else {
            Err(meta.error("unsupportted tea property"))
        }
    }
}

#[proc_macro_attribute]
pub fn tea(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut attrs = TeaAttributes::default();

    let tea_parser = syn::meta::parser(|meta| attrs.parse(meta));

    parse_macro_input!(args with tea_parser);

    println!(
        "kind={:?}, hot={:?}, with={:?}",
        attrs
            .kind
            .unwrap()
            .token(),
        attrs.hot,
        attrs
            .with
            .len(),
    );

    input
}
