use darling::FromDeriveInput;
use quote::ToTokens;
use syn::parse::{Parse, Parser};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(usecase), forward_attrs(allow, doc, cfg))]
#[darling(supports(any))]
pub(super) struct UseCase {
    ident: syn::Ident,
    generics: syn::Generics,
    #[darling(default)]
    input: Option<syn::Type>,
    #[darling(default)]
    output: Option<syn::Type>,
}

impl ToTokens for UseCase {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let UseCase {
            ref ident,
            ref generics,
            ref input,
            ref output,
        } = *self;

        let (imp, ty, wher) = generics.split_for_impl();

        let input = input
            .clone()
            .unwrap_or(syn::Type::parse.parse_str("()").unwrap());

        let output = output
            .clone()
            .unwrap_or(syn::Type::parse.parse_str("()").unwrap());

        tokens.extend(quote::quote! {
            impl #imp UseCase for #ident #ty #wher {
                type Input = #input;
                type Output = #output;
            }
        });
    }
}
