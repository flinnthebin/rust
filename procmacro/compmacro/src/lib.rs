/* ---Legend---
*  +: 1 or more
*  *: 0 or more
*
* comp: mapping for_if_clause+
*
* mapping: expression
*
* for_if_clause:
*     | 'for' pattern 'in' expression ('if' expression)*
*
* pattern: name (, name)*
*
*/

/*
* comp is a mapping followed by a for_if_clause. it has both which implies a product type
* which we can represent as a struct. if it was one or the other, mapping or for_if_clause
* that would imply a sum type so an enum.
*/
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    Token,
    parse::{Parse, ParseStream},
};

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}
impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}
impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream) {}
}

struct Mapping(syn::Expr);
impl syn::parse::Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
        // Ok(Self(input.parse()?))                         <=== it's a little pythonic
    }
}

struct ForIfClause {
    pattern: Pattern,
    expression: syn::Expr,
    conditions: Vec<Condition>,
}
impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<Token![for]>()?;
        let pattern: Pattern = input.parse()?;
        _ = input.parse::<Token![in]>()?;
        let expression: syn::Expr = input.parse()?;
        let conditions: Vec<Condition> = parse_zero_or_more(input);
        Ok(Self {
            pattern,
            expression,
            conditions,
        })
    }
}
fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result: Vec<T> = Vec::new();
    while let Ok(item) = input.parse::<T>() {
        result.push(item);
    }
    result
}

struct Pattern(syn::Pat);
impl syn::parse::Parse for Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
        //Ok(Self(syn::pat::parse_single(input)?))          <=== can also be written like this
    }
}

struct Condition(syn::Expr);
impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        _ = input.parse::<syn::Token![if]>()?;
        // Let _: Token![if] = input.parse()?;              <=== noob-friendly notation
        input.parse::<syn::Expr>().map(Self)
        //input.parse().map(Self)                           <=== can be written without turbofish bc rust typesystem is *fancy*
    }
}
