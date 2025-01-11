use std::collections::HashSet;

use quote::{
    quote,
    ToTokens,
};
use syn::{
    fold,
    fold::Fold,
    parse::Parse,
    parse_macro_input,
    parse_quote,
    punctuated::Punctuated,
    BinOp,
    Expr,
    Ident,
    ItemFn,
    Local,
    Pat,
    Stmt,
    Token,
};

use syn::ExprRange;
use syn::Path;

fn is_assign_op(op: BinOp) -> bool {
    match op {
        BinOp::AddAssign(_)
        | BinOp::SubAssign(_)
        | BinOp::MulAssign(_)
        | BinOp::DivAssign(_)
        | BinOp::RemAssign(_)
        | BinOp::BitXorAssign(_)
        | BinOp::BitAndAssign(_)
        | BinOp::BitOrAssign(_)
        | BinOp::ShlAssign(_)
        | BinOp::ShrAssign(_) => true,
        _ => false,
    }
}

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars
                .into_iter()
                .collect(),
        })
    }
}

impl Fold for Args {
    fn fold_expr(&mut self, e: syn::Expr) -> syn::Expr {
        todo!()
    }

    fn fold_stmt(&mut self, e: syn::Stmt) -> syn::Stmt {
        todo!()
    }
}

impl Args {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
                if e.path
                    .leading_colon
                    .is_some()
                {
                    false
                } else if e
                    .path
                    .segments
                    .len()
                    != 1
                {
                    false
                } else {
                    let first = e
                        .path
                        .segments
                        .first()
                        .unwrap();
                    self.vars
                        .contains(&first.ident)
                        && first
                            .arguments
                            .is_empty()
                }
            }
            _ => false,
        }
    }

    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
            Pat::Ident(ref p) => self
                .vars
                .contains(&p.ident),
            _ => false,
        }
    }

    fn assign_and_print(
        &mut self,
        left: Expr,
        op: &dyn ToTokens,
        right: Expr,
    ) -> Expr {
        let right = fold::fold_expr(self, right);
        parse_quote!(
            #left #op #right;
            println!(concat!(stringify!(#left)," = {:?}"), #left);
        )
    }

    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(
            *init
                .unwrap()
                .expr,
        );
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };

        parse_quote!(
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat(stringify(#ident), " = {:?} "), #ident);
                #ident+
            }
        )
    }
}

#[proc_macro_attribute]
pub fn trace_var(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut args = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as ItemFn);

    let output = args.fold_item_fn(input);
    proc_macro::TokenStream::from(quote!(#output))
}
