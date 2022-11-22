use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::fold;
use syn::fold::Fold;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_quote;
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::Ident;
use syn::Local;
use syn::Pat;
use syn::Result;
use syn::Stmt;
use syn::{parse_macro_input, ItemFn, Token};

#[proc_macro_attribute]
pub fn my_custom_attribute(_metadata: TokenStream, _input: TokenStream) -> TokenStream {
    TokenStream::from(quote! {struct H{}})
}

struct Args {
    vars: HashSet<Ident>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

impl Fold for Args {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Assign(e) => {
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.eq_token, *e.right)
                } else {
                    Expr::Assign(fold::fold_expr_assign(self, e))
                }
            }
            Expr::AssignOp(e) => {
                if self.should_print_expr(&e.left) {
                    self.assign_and_print(*e.left, &e.op, *e.right)
                } else {
                    Expr::AssignOp(fold::fold_expr_assign_op(self, e))
                }
            }
            _ => fold::fold_expr(self, e),
        }
    }
    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(s) => {
                if s.init.is_some() && self.should_print_pat(&s.pat) {
                    self.let_and_print(s)
                } else {
                    Stmt::Local(fold::fold_local(self, s))
                }
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

impl Args {
    fn should_print_expr(&self, e: &Expr) -> bool {
        match *e {
            Expr::Path(ref e) => {
                if e.path.leading_colon.is_some() {
                    false
                } else if e.path.segments.len() != 1 {
                    false
                } else {
                    let first = e.path.segments.first().unwrap();
                    self.vars.contains(&first.ident) && first.arguments.is_empty()
                }
            }
            _ => false,
        }
    }
    fn should_print_pat(&self, p: &Pat) -> bool {
        match p {
            Pat::Ident(ref p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }

    fn assign_and_print(&mut self, left: Expr, op: &dyn ToTokens, right: Expr) -> Expr {
        let right = fold::fold_expr(self, right);
        parse_quote!(
            #left #op #right
            println!(concat!(stringify!(#left), " = {:?}"), #left);
        )
    }
    fn let_and_print(&mut self, local: Local) -> Stmt {
        let Local { pat, init, .. } = local;
        let init = self.fold_expr(*init.unwrap().1);
        let ident = match pat {
            Pat::Ident(ref p) => &p.ident,
            _ => unreachable!(),
        };
        parse_quote! {
            let #pat = {
                #[allow(unused_mut)]
                let #pat = #init;
                println!(concat!(stringify!(#ident), " = {:?}"), #ident);
                #ident
            };
        }
    }
}

#[proc_macro_attribute]
pub fn trace_vars(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);
    let mut args = parse_macro_input!(metadata as Args);

    let output = args.fold_item_fn(input);

    TokenStream::from(quote! {#output})
}
