use darling::{FromDeriveInput, ToTokens};

mod entity;
mod util;
mod value_type;

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

#[proc_macro_error::proc_macro_error]
#[proc_macro_derive(Entity, attributes(id_field, created_at_field))]
pub fn derive_entity(
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    match entity::Entity::from_derive_input(&input) {
        | Ok(tokens) => tokens,
        | Err(err) => panic!("{err:?}"),
    }
    .into_token_stream()
    .into()
}
