use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use sexplib::parser;

#[proc_macro]
pub fn expr(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();
    let path = match tokens.next() {
        | Some(TokenTree::Group(g)) => g.stream(),
        | _ => panic!("Invalid input")
    };
    let path = proc_macro2::TokenStream::from(path);
    let input = tokens.collect::<TokenStream>();
    let expr = parser::sexp::parse_expr(&input.to_string()).unwrap().1;
    
    let exp = quote!({
        use #path::{self as __expr};
        #expr
    });
    
    TokenStream::from(exp)
}
