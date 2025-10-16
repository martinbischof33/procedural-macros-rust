// comp: mapping for_if_clause
// mapping: sequence
// for_if_clause:
//     | for' pattern 'in' sequence ('if' expression)*
// pattern: name (, name) *
//

use quote::quote;
use syn::{parse::Parse, parse_macro_input};

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl syn::parse::Parse for Comp {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: Mapping::parse(input)?,
            for_if_clause: ForIfClause::parse(input)?,
        })
    }
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        tokens.extend(quote! {
            core::itter::IntoIterator::into_iter(#sequence).filter_map(move |#pattern|) {
                (true #(&& (#conditions))*).then(|| #mapping)
            }
        });
    }
}

struct Mapping(syn::Expr);

impl syn::parse::Parse for Mapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
        // syn::Expr::parse(input).map(Self) equivalent
    }
}

impl quote::ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: syn::Expr,
    conditions: Vec<Condition>,
}

impl syn::parse::Parse for ForIfClause {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = <syn::Token![for]>::parse(input)?;
        let pattern: Pattern = Pattern::parse(input)?;
        _ = <syn::Token![in]>::parse(input)?;
        let sequence: syn::Expr = syn::Expr::parse(input)?;
        let conditions: Vec<Condition> = vec![];
        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

struct Pattern(syn::Pat);

impl syn::parse::Parse for Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        syn::Pat::parse_single(input).map(Self)
        // same as Ok(Self(syn::Pat::parse_single(input)?))
    }
}

impl quote::ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(syn::Expr);

impl syn::parse::Parse for Condition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        _ = <syn::Token![if]>::parse(input)?;
        syn::Expr::parse(input).map(Self)
    }
}

impl quote::ToTokens for Condition {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let composition = parse_macro_input!(input as Comp);
    quote! {#composition}.into()
}
