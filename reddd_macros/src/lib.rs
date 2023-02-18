use darling::{FromDeriveInput, ToTokens};

mod value_type;
mod util;

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(ValueType, attributes(main_field))]
pub fn derive_value_type(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match value_type::ValueType::from_derive_input(&input) {
        | Ok(tokens) => tokens,
        | Err(err) => panic!("{err:?}"),
    }
    .into_token_stream()
    .into()
}
