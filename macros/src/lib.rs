use proc_macro::{TokenStream};
use proc_macro2::TokenTree;
use quote::{ToTokens, quote};

use syn::{Token, Result, parse_macro_input};
use syn::Item;
use syn::parse::{Parse, ParseStream};

use std::time::{Duration, Instant};

struct PrintOption {
    printer: Printer,
    time_unit: TimeUnit,
    original_function_name: String,
    original_function: Token![fn],
}
impl PrintOption {
    fn parse(attr: TokenStream, function: TokenStream) {

    }
    fn parse_attributes(attr: TokenStream) {
        let attr = proc_macro2::TokenStream::from(attr);
        for (index, token_tree) in attr.into_iter().enumerate() {
            println!("# Token {}: {:#?}", index, token_tree);
            if index % 2 == 1 { // Delimiter checker
                let is_comma = Self::token_is_comma(token_tree);
                if !is_comma {
                    panic!("Commas(,) are used as attribute delimiter.")
                }
                continue;
            }

            match token_tree {
                TokenTree::Ident(ident) => {

                },
                TokenTree::Literal(literal) => {

                },
                TokenTree::Group(group) => {

                },
                _ => {
                    panic!("Unknown attributes")
                },
            }            
        }
    }
    fn token_is_comma(token_tree: TokenTree) -> bool {
        if let TokenTree::Punct(punct) = token_tree {
            if punct.to_string() == "," {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

enum Printer {
    StdOut,
    StdErr,
    Both,
}

impl Printer {
    fn parse(tag: String) -> Self {
        match tag.as_ref() {
            "stdout" => {
                Self::StdOut
            },
            "stderr" => {
                Self::StdErr
            },
            "both" => {
                Self::Both
            },
            _ => {
                panic!("First attribute only allows stdout, stderr or both")
            },
        }
    }
}

enum TimeUnit {
    S,
    Ms,
    Us,
    Ns,
}

/// Attributes
/// (1) stdout, stderr, both
/// (2) s, ms, us, ns
/// (3) [features]
#[proc_macro_attribute]
pub fn test_macro(attr: TokenStream, function: TokenStream) -> TokenStream {
    let test = PrintOption::parse_attributes(attr);

    let original_function: syn::ItemFn = parse_macro_input!(function);
    let original_function_name = original_function.sig.ident.to_string();

    let attrs = &original_function.attrs;
    let vis = &original_function.vis;
    let constness = &original_function.sig.constness;
    let asyncness = &original_function.sig.asyncness;
    let unsafety = &original_function.sig.unsafety;
    let abi = &original_function.sig.abi;
    let ident = &original_function.sig.ident;
    let generics = &original_function.sig.generics;
    let inputs = &original_function.sig.inputs;
    let variadic = &original_function.sig.variadic;
    let output = &original_function.sig.output;
    let block = &original_function.block;
    
    let tokens = quote! {
        #(#attrs),*
        #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
            let start = Instant::now();
            let result = #block;
            let duration = start.elapsed();
            println!("{:?}, {:?}", #original_function_name, duration);
            result
        }
    };

    tokens.into()
}
