extern crate proc_macro;

use lazy_static::lazy_static;
use proc_macro::TokenStream;
use quote::quote;
use std::sync::Mutex;
use syn::{parse_macro_input, DeriveInput};

lazy_static! {
    static ref ERROR_STRUCTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// The ErrorProcessor struct is responsible for registering error structs and
/// generating the ErrorKind enum.
struct ErrorProcessor;

impl ErrorProcessor {
    /// Register the struct name in a global list.
    ///
    /// # Arguments
    ///
    /// * `input` - The parsed input of the derive macro, representing a struct.
    fn register(input: DeriveInput) {
        let name = input.ident.to_string();

        let mut structs = ERROR_STRUCTS.lock().expect("Failed to lock the mutex.");

        if !structs.contains(&name) {
            structs.push(name);
        }
    }

    /// Generate the ErrorKind enum and From trait implementations from
    /// registered struct names.
    ///
    /// # Returns
    ///
    /// * A `TokenStream` representing the generated ErrorKind enum and trait
    ///   implementations.
    fn generate() -> TokenStream {
        let structs = ERROR_STRUCTS.lock().expect("Failed to lock the mutex.");

        let (variants, from_impls): (Vec<_>, Vec<_>) = structs
            .iter()
            .map(|name| {
                let ident = syn::Ident::new(name, proc_macro2::Span::call_site());
                (
                    quote! { #ident },
                    quote! {
                        impl From<#ident> for ErrorKind {
                            fn from(_error: #ident) -> ErrorKind {
                                ErrorKind::#ident
                            }
                        }
                    },
                )
            })
            .unzip();

        let expanded = quote! {
            #[derive(Debug)]
            pub enum ErrorKind {
                #(#variants),*
            }

            #(#from_impls)*
        };

        TokenStream::from(expanded)
    }
}

/// Procedural macro to derive ErrorKind for a struct.
/// This macro registers the struct name in a global list.
///
/// # Arguments
///
/// * `input` - The input tokens of the derive macro, representing a struct.
///
/// # Returns
///
/// * A `TokenStream` that is empty, as registration does not produce output.
#[proc_macro_derive(ErrorKind)]
pub fn error_kind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    ErrorProcessor::register(input);

    TokenStream::new()
}

/// Procedural macro to generate the ErrorKind enum and trait implementations
/// from registered struct names.
///
/// # Arguments
///
/// * `_input` - The input tokens of the macro, which are not used.
///
/// # Returns
///
/// * A `TokenStream` representing the generated ErrorKind enum and trait
///   implementations.
#[proc_macro]
pub fn generate_error_kind(_input: TokenStream) -> TokenStream { ErrorProcessor::generate() }
