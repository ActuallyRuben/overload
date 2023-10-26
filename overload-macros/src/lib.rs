#![feature(proc_macro_quote)]

use proc_macro::{quote, TokenStream};
use std::default::Default;

use syn::{Block, braced, FnArg, Generics, Ident, parenthesized, parse_macro_input, parse_quote, Pat, ReturnType, Token, Type};
use syn::__private::ToTokens;
use syn::FnArg::Typed;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::{Brace, Paren};

struct Signature {
    generics: Generics,
    _paren_token: Paren,
    args: Punctuated<FnArg, Token![,]>,
    return_type: ReturnType,
    body: Block,
}

impl Parse for Signature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args;
        Ok(Self {
            generics: input.parse()?,
            _paren_token: parenthesized!(args in input),
            args: Punctuated::parse_terminated(&args)?,
            return_type: input.parse()?,
            body: input.parse()?,
        })
    }
}

struct OverloadInput {
    public: Option<Token![pub]>,
    _function: Token![fn],
    name: Ident,
    _brace_token: Brace,
    signatures: Vec<Signature>,
}

impl Parse for OverloadInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let signature_tokens;
        let public = input.parse()?;
        let function = input.parse()?;
        let name = input.parse()?;
        let brace_token = braced!(signature_tokens in input);
        let mut signatures = Vec::new();
        while !signature_tokens.is_empty() {
            signatures.push(signature_tokens.parse()?)
        }
        Ok(Self {
            public,
            _function: function,
            name,
            _brace_token: brace_token,
            signatures,
        })
    }
}

fn overload_signature(name: TokenStream, signature: Signature) -> TokenStream {
    let loose_ret_type = match signature.return_type {
        ReturnType::Default => parse_quote!(()),
        ReturnType::Type(_, ref t) => { t.as_ref().clone() }
    }.into_token_stream();
    let generics = signature.generics.into_token_stream();
    let ret_type = signature.return_type.into_token_stream();
    let body = signature.body.into_token_stream();
    let mut arg_pattern: Punctuated<Pat, Token![,]> = Punctuated::new();
    let mut arg_types: Punctuated<Type, Token![,]> = Punctuated::new();

    for arg in signature.args.into_iter() {
        if let Typed(arg) = arg {
            arg_pattern.push(*(arg.pat));
            arg_types.push(*(arg.ty))
        } else {
            panic!("self is not supported")
        }
    }
    if !arg_pattern.is_empty() {
        arg_pattern.push_punct(Default::default());
    }
    if !arg_types.is_empty() {
        arg_types.push_punct(Default::default());
    }

    let arg_pattern = arg_pattern.into_token_stream();
    let arg_types = arg_types.into_token_stream();
    return quote! {
        impl$generics FnOnce<($arg_types)> for $name {
            type Output = $loose_ret_type;

            extern "rust-call" fn call_once(self, ($arg_pattern): ($arg_types)) $ret_type $body
        }
    }
}


#[proc_macro]
pub fn overload(tokens: TokenStream) -> TokenStream {
    let overload_data = parse_macro_input!(tokens as OverloadInput);
    let public = overload_data.public.into_token_stream();
    let name = overload_data.name.into_token_stream();
    let result: TokenStream = quote! {
        #[allow(non_camel_case_types)]
        $public struct $name;
    };

    let stream = [result].into_iter()
        .chain(
            overload_data.signatures.into_iter()
                .map(move |s| overload_signature(name.clone().into(), s))
        );
    TokenStream::from_iter(stream)
}