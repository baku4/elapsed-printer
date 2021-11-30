use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;

use std::collections::HashSet;

struct PrintOption {
    printer: Printer,
    time_unit: TimeUnit,
    features: Vec<String>,
    function_name: String,
    original_function: syn::ItemFn,
}
impl PrintOption {
    // Parse from token stream
    fn parse(attr: TokenStream, function: TokenStream) -> Self {
        let (printer, time_unit, features) = Self::parse_attributes(attr);
        let (function_name, original_function) = Self::parse_function(function);

        Self {
            printer,
            time_unit,
            features,
            function_name,
            original_function,
        }
    }
    fn parse_function(function: TokenStream) -> (String, syn::ItemFn) {
        let original_function: syn::ItemFn = match syn::parse(function) {
            Ok(item_fn) => item_fn,
            Err(err) => panic!("{}", err)
        };
        let function_name = original_function.sig.ident.to_string();

        (function_name, original_function)
    }
    fn parse_attributes(attr: TokenStream) -> (Printer, TimeUnit, Vec<String>) {
        let mut optional_printer: Option<Printer> = None;
        let mut optional_time_unit: Option<TimeUnit> = None;
        let mut features = Vec::new();

        let attr = proc_macro2::TokenStream::from(attr);
        for (index, token_tree) in attr.into_iter().enumerate() {
            if index % 2 == 1 { // Delimiter checker
                let is_comma = Self::token_is_comma(token_tree);
                if !is_comma {
                    panic!("Commas(,) are used as attribute delimiter.")
                }
                continue;
            }

            match token_tree {
                TokenTree::Ident(ident) => {
                    let tag = ident.to_string();
                    Self::parse_from_tag(tag, &mut optional_printer, &mut optional_time_unit);
                },
                TokenTree::Literal(literal) => {
                    let tag = literal.to_string();
                    let tag = tag.replace("\"", ""); // Remove quotes
                    Self::parse_from_tag(tag, &mut optional_printer, &mut optional_time_unit);
                },
                TokenTree::Group(group) => {
                    features = Self::parse_features(group);
                },
                _ => {
                    panic!("Unknown attributes")
                },
            }            
        }

        let printer = match optional_printer {
            Some(printer) => printer,
            None => Printer::default(),
        };
        let time_unit = match optional_time_unit {
            Some(time_unit) => time_unit,
            None => TimeUnit::default(),
        };

        (printer, time_unit, features)
    }
    fn parse_from_tag(
        tag: String,
        optional_printer: &mut Option<Printer>,
        optional_time_unit: &mut Option<TimeUnit>,
    ) {
        // Parse printer
        let printer = match tag.as_ref() {
            "stdout" => Some(Printer::StdOut),
            "stderr" => Some(Printer::StdErr),
            "both" => Some(Printer::Both),
            _ => None,
        };
        if printer.is_some() {
            if optional_printer.is_none() {
                *optional_printer = printer;
                return
            } else {
                panic!("Printer attribute is assigned multiple times.")
            }
        }

        // Parse time unit
        let time_unit = match tag.as_ref() {
            "auto" => Some(TimeUnit::Auto),
            "s" => Some(TimeUnit::S),
            "ms" => Some(TimeUnit::Ms),
            "us" => Some(TimeUnit::Us),
            "ns" => Some(TimeUnit::Ns),
            _ => {
                panic!("Attribute allows printer settings(stdout, stderr, both) and time unit(auto, s, ms, us, ns)")
            }
        };
        if time_unit.is_some() {
            if optional_time_unit.is_none() {
                *optional_time_unit = time_unit;
                return
            } else {
                panic!("Time unit attribute is assigned multiple times.")
            }
        }
    }
    fn parse_features(group: proc_macro2::Group) -> Vec<String> {
        let mut features_set = HashSet::new();

        for (index, token_tree) in group.stream().into_iter().enumerate() {
            if index % 2 == 1 {
                let is_comma = Self::token_is_comma(token_tree);
                if !is_comma {
                    panic!("Commas(,) are used as feature delimiter.")
                }
                continue;
            }

            let feature = match token_tree {
                TokenTree::Ident(ident) => {
                    ident.to_string()
                },
                TokenTree::Literal(literal) => {
                    let tag = literal.to_string();
                    tag.replace("\"", "")
                },
                _ => {
                    panic!("Features allows only attribute");
                },
            };
            features_set.insert(feature);
        }

        features_set.into_iter().collect()
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

    // Generate new token stream
    fn new_token_stream(&self) -> TokenStream {
        let function_name = &self.function_name;

        let attrs = &self.original_function.attrs;
        let vis = &self.original_function.vis;
        let constness = &self.original_function.sig.constness;
        let asyncness = &self.original_function.sig.asyncness;
        let unsafety = &self.original_function.sig.unsafety;
        let abi = &self.original_function.sig.abi;
        let ident = &self.original_function.sig.ident;
        let generics = &self.original_function.sig.generics;
        let inputs = &self.original_function.sig.inputs;
        let variadic = &self.original_function.sig.variadic;
        let output = &self.original_function.sig.output;
        let block = &self.original_function.block;

        let duration_token = self.time_unit.duration_token();
        let print_token = self.printer.print_token(&self.time_unit, function_name);

        let features = &self.features;

        let tokens = if features.len() == 0 {
            quote! {
                #(#attrs),*
                #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
                    let start = std::time::Instant::now();
                    let result = #block;
                    #duration_token
                    #print_token
                    result
                }
            }
        } else {
            quote! {
                #(#attrs),*
                #vis #constness #asyncness #unsafety #abi fn #ident #generics(#inputs #variadic) #output {
                    #[cfg(any(#(feature=#features),*))]
                    {
                        let start = std::time::Instant::now();
                        let result = #block;
                        #duration_token
                        #print_token
                        result
                    }
                    #[cfg(not(any(#(feature=#features),*)))]
                    #block
                }
            }
        };

        tokens.into()
    }
}

enum Printer {
    StdOut,
    StdErr,
    Both,
}
impl Default for Printer {
    fn default() -> Self {
        Self::StdOut
    }
}
impl Printer {
    fn print_token(&self, time_unit: &TimeUnit, function_name: &String) -> TokenStream2 {
        match self {
            Self::StdOut => {
                time_unit.print_to_stdout_token(function_name)
            },
            Self::StdErr => {
                time_unit.print_to_stderr_token(function_name)
            },
            Self::Both => {
                let print_to_stdout_token = time_unit.print_to_stdout_token(function_name);
                let print_to_stderr_token = time_unit.print_to_stderr_token(function_name);
                quote! {
                    #print_to_stdout_token
                    #print_to_stderr_token
                }
            },
        }
    }
}

enum TimeUnit {
    Auto,
    S,
    Ms,
    Us,
    Ns,
}
impl Default for TimeUnit {
    fn default() -> Self {
        Self::Auto
    }
}
impl TimeUnit {
    fn duration_token(&self) -> TokenStream2 {
        match self {
            Self::Auto => {
                quote! {
                    let duration = start.elapsed();
                }
            },
            Self::S => {
                quote! {
                    let duration = start.elapsed().as_secs();
                }
            },
            Self::Ms => {
                quote! {
                    let duration = start.elapsed().as_millis();
                }
            },
            Self::Us => {
                quote! {
                    let duration = start.elapsed().as_micros();
                }
            },
            Self::Ns => {
                quote! {
                    let duration = start.elapsed().as_nanos();
                }
            },
        }
    }
    fn print_to_stdout_token(&self, function_name: &String) -> TokenStream2 {
        match self {
            Self::Auto => {
                quote! {
                    println!("{}, {:?}", #function_name, duration);
                }
            },
            Self::S => {
                quote! {
                    println!("{}, {}s", #function_name, duration);
                }
            },
            Self::Ms => {
                quote! {
                    println!("{}, {}ms", #function_name, duration);
                }
            },
            Self::Us => {
                quote! {
                    println!("{}, {}us", #function_name, duration);
                }
            },
            Self::Ns => {
                quote! {
                    println!("{}, {}ns", #function_name, duration);
                }
            },
        }
    }
    fn print_to_stderr_token(&self, function_name: &String) -> TokenStream2 {
        match self {
            Self::Auto => {
                quote! {
                    eprintln!("{}, {:?}", #function_name, duration);
                }
            },
            Self::S => {
                quote! {
                    eprintln!("{}, {}s", #function_name, duration);
                }
            },
            Self::Ms => {
                quote! {
                    eprintln!("{}, {}ms", #function_name, duration);
                }
            },
            Self::Us => {
                quote! {
                    eprintln!("{}, {}us", #function_name, duration);
                }
            },
            Self::Ns => {
                quote! {
                    eprintln!("{}, {}ns", #function_name, duration);
                }
            },
        }
    }
}

/// Attributes
/// (1) stdout, stderr, both (default: stdout)
/// (2) auto, s, ms, us, ns (default: auto)
/// (3) [features] (e.g. [feature_1, "feature_2", feature_3])
#[proc_macro_attribute]
pub fn print_elapsed(attr: TokenStream, function: TokenStream) -> TokenStream {
    let print_option = PrintOption::parse(attr, function);
    let new_token_stream = print_option.new_token_stream();
    new_token_stream
}
